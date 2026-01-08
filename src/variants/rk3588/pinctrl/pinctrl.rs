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

    /// 配置单个引脚（完整配置）
    ///
    /// 一次性配置引脚的功能、上下拉和驱动强度。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `config` - 引脚配置
    pub fn config_pin(&self, pin: PinId, config: &PinConfig) -> PinctrlResult<()> {
        self.set_mux(pin, config.function)?;
        if let Some(pull) = config.pull {
            self.set_pull(pin, pull)?;
        }
        if let Some(drive) = config.drive {
            self.set_drive(pin, drive)?;
        }
        Ok(())
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
        let value = function.raw() << config.bit_offset;

        unsafe {
            let reg_ptr = self.ioc_base.as_ptr().add(config.reg_offset) as *mut u32;
            reg_ptr.write_volatile((mask << 16) | value);
        }

        // 如果需要双寄存器配置（GPIO0 的某些引脚）
        if let Some(extra) = extra_config {
            let mask = 0xfu32 << extra.bit_offset;
            let value = function.raw() << extra.bit_offset;

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
}
