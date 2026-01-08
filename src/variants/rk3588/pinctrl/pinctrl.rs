//! RK3588 Pinctrl 驱动实现
//!
//! 提供引脚复用和引脚配置功能。

use crate::{
    Mmio, PinId,
    pinctrl::{DriveStrength, Function, PinConfig, PinctrlError, PinctrlResult, Pull},
};
use core::ptr::NonNull;

/// RK3588 Pinctrl 驱动
///
/// 管理引脚复用和配置功能，支持引脚功能选择、上下拉和驱动强度配置。
///
/// # 示例
///
/// ```ignore
/// use rockchip_soc::rk3588::Pinctrl;
/// use rockchip_soc::pinctrl::{PinId, PinConfig, Function, Pull, DriveStrength};
///
/// // 初始化 pinctrl
/// let pinctrl = unsafe { Pinctrl::new(0xfd58a000) };
///
/// // 配置 UART0 引脚
/// let tx_pin = PinId::new(32).unwrap();
/// pinctrl.config_pin(tx_pin, &PinConfig::new(Function::Alt1)
///     .with_pull(Pull::Disabled)
///     .with_drive(DriveStrength::Ma8));
/// ```
pub struct Pinctrl {
    /// IOC 基地址
    ioc_base: NonNull<u8>,
}

unsafe impl Send for Pinctrl {}

impl Pinctrl {
    /// 创建新的 pinctrl 实例
    ///
    /// # 参数
    ///
    /// * `ioc_base` - IOC 寄存器基地址
    ///
    /// # Safety
    ///
    /// `ioc_base` 必须是有效的 IOC 寄存器基地址，并且在整个生命周期内保持有效。
    pub unsafe fn new(ioc_base: Mmio) -> Self {
        Self { ioc_base }
    }

    /// 设置引脚功能（pinmux）
    ///
    /// 配置引脚的复用功能（GPIO、UART、SPI 等）。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `function` - 引脚功能
    ///
    /// # 参考
    ///
    /// u-boot: `drivers/pinctrl/rockchip/pinctrl-rk3588.c:rk3588_set_mux()`
    pub fn set_mux(&self, pin: PinId, function: Function) -> PinctrlResult<()> {
        use crate::variants::rk3588::pinctrl::iomux::calc_iomux_config;

        let (config, extra_config) =
            calc_iomux_config(pin).ok_or(PinctrlError::InvalidPinId(pin.raw()))?;

        // Rockchip 写掩码机制：高16位清除，低16位设置
        // 每个引脚占 4 位，掩码为 0xf
        let mask = 0xfu32 << config.bit_offset;
        let value = function.num() << config.bit_offset;

        unsafe {
            let reg_ptr = self.ioc_base.as_ptr().add(config.reg_offset) as *mut u32;
            reg_ptr.write_volatile((mask << 16) | value);
        }

        // 如果需要双寄存器配置（GPIO0 的某些引脚）
        if let Some(extra) = extra_config {
            let mask = 0xfu32 << extra.bit_offset;
            let value = function.num() << extra.bit_offset;

            unsafe {
                let reg_ptr = self.ioc_base.as_ptr().add(extra.reg_offset) as *mut u32;
                reg_ptr.write_volatile((mask << 16) | value);
            }
        }

        Ok(())
    }

    /// 设置 pull 配置
    ///
    /// 配置引脚的上下拉电阻。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `pull` - 上下拉配置
    ///
    /// # 参考
    ///
    /// u-boot: `drivers/pinctrl/rockchip/pinctrl-rk3588.c:rk3588_set_pull()`
    pub fn set_pull(&self, pin: PinId, pull: Pull) -> PinctrlResult<()> {
        use crate::variants::rk3588::pinctrl::pinconf_regs::find_pull_entry;

        let (reg_offset, bit_offset) =
            find_pull_entry(pin).ok_or(PinctrlError::InvalidPinId(pin.raw()))?;

        // Rockchip 写掩码机制
        // 每个 pull 配置占 2 位，掩码为 0x3
        let mask = 0x3u32 << bit_offset;
        let value = (pull as u32) << bit_offset;

        unsafe {
            let reg_ptr = self.ioc_base.as_ptr().add(reg_offset) as *mut u32;
            reg_ptr.write_volatile((mask << 16) | value);
        }

        Ok(())
    }

    /// 设置 drive strength
    ///
    /// 配置引脚输出驱动强度。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `drive` - 驱动强度配置
    ///
    /// # 参考
    ///
    /// u-boot: `drivers/pinctrl/rockchip/pinctrl-rk3588.c:rk3588_set_drive()`
    pub fn set_drive(&self, pin: PinId, drive: DriveStrength) -> PinctrlResult<()> {
        use crate::variants::rk3588::pinctrl::pinconf_regs::find_drive_entry;

        let (reg_offset, bit_offset) =
            find_drive_entry(pin).ok_or(PinctrlError::InvalidPinId(pin.raw()))?;

        // Rockchip 写掩码机制
        // 每个 drive 配置占 8 位（但实际只使用低 2 位）
        let mask = 0x3u32 << bit_offset;
        let value = (drive as u32) << bit_offset;

        unsafe {
            let reg_ptr = self.ioc_base.as_ptr().add(reg_offset) as *mut u32;
            reg_ptr.write_volatile((mask << 16) | value);
        }

        Ok(())
    }

    /// 读取引脚功能（pinmux）
    ///
    /// 读取引脚当前的复用功能配置。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    ///
    /// # 返回
    ///
    /// 返回引脚当前的功能配置
    pub fn get_mux(&self, pin: PinId) -> PinctrlResult<Function> {
        use crate::variants::rk3588::pinctrl::iomux::calc_iomux_config;

        let (config, _extra_config) =
            calc_iomux_config(pin).ok_or(PinctrlError::InvalidPinId(pin.raw()))?;

        // 读取寄存器值
        let reg_value = unsafe {
            let reg_ptr = self.ioc_base.as_ptr().add(config.reg_offset) as *const u32;
            reg_ptr.read_volatile()
        };

        log::info!(
            "get_mux: pin={}, reg_offset={:#x}, bit_offset={}, reg_value={:#x}",
            pin.raw(),
            config.reg_offset,
            config.bit_offset,
            reg_value
        );

        // 提取功能配置字段（每个引脚占 4 位）
        let mask = 0xfu32 << config.bit_offset;
        let func_num = (reg_value & mask) >> config.bit_offset;

        log::info!("get_mux: func_num={}, mask={:#x}", func_num, mask);

        // 转换为 Function 枚举
        Function::from_num(func_num).ok_or(PinctrlError::InvalidConfig)
    }

    /// 读取 pull 配置
    ///
    /// 读取引脚当前的上下拉配置。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    ///
    /// # 返回
    ///
    /// 返回引脚当前的上下拉配置
    pub fn get_pull(&self, pin: PinId) -> PinctrlResult<Pull> {
        use crate::variants::rk3588::pinctrl::pinconf_regs::find_pull_entry;

        let (reg_offset, bit_offset) =
            find_pull_entry(pin).ok_or(PinctrlError::InvalidPinId(pin.raw()))?;

        // 读取寄存器值
        let reg_value = unsafe {
            let reg_ptr = self.ioc_base.as_ptr().add(reg_offset) as *const u32;
            reg_ptr.read_volatile()
        };

        log::info!(
            "get_pull: pin={}, reg_offset={:#x}, bit_offset={}, reg_value={:#x}",
            pin.raw(),
            reg_offset,
            bit_offset,
            reg_value
        );

        // 提取 pull 配置字段（每个 pull 占 2 位）
        let mask = 0x3u32 << bit_offset;
        let pull_value = (reg_value & mask) >> bit_offset;

        log::info!("get_pull: pull_value={}, mask={:#x}", pull_value, mask);

        // 转换为 Pull 枚举
        match pull_value {
            0 => Ok(Pull::Disabled),
            1 => Ok(Pull::PullUp),
            2 => Ok(Pull::PullDown),
            _ => {
                log::warn!("Invalid pull value {} for pin {}", pull_value, pin.raw());
                Err(PinctrlError::InvalidConfig)
            }
        }
    }

    /// 读取 drive strength
    ///
    /// 读取引脚当前的驱动强度配置。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    ///
    /// # 返回
    ///
    /// 返回引脚当前的驱动强度配置
    pub fn get_drive(&self, pin: PinId) -> PinctrlResult<DriveStrength> {
        use crate::variants::rk3588::pinctrl::pinconf_regs::find_drive_entry;

        let (reg_offset, bit_offset) =
            find_drive_entry(pin).ok_or(PinctrlError::InvalidPinId(pin.raw()))?;

        // 读取寄存器值
        let reg_value = unsafe {
            let reg_ptr = self.ioc_base.as_ptr().add(reg_offset) as *const u32;
            reg_ptr.read_volatile()
        };

        log::info!(
            "get_drive: pin={}, reg_offset={:#x}, bit_offset={}, reg_value={:#x}",
            pin.raw(),
            reg_offset,
            bit_offset,
            reg_value
        );

        // 提取 drive 配置字段（每个 drive 占 2 位）
        let mask = 0x3u32 << bit_offset;
        let drive_value = (reg_value & mask) >> bit_offset;

        log::info!("get_drive: drive_value={}, mask={:#x}", drive_value, mask);

        // 转换为 DriveStrength 枚举
        match drive_value {
            0 => Ok(DriveStrength::Ma2),
            1 => Ok(DriveStrength::Ma4),
            2 => Ok(DriveStrength::Ma8),
            3 => Ok(DriveStrength::Ma12),
            _ => {
                log::warn!("Invalid drive value {} for pin {}", drive_value, pin.raw());
                Err(PinctrlError::InvalidConfig)
            }
        }
    }
}
