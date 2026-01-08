//! RK3588 PinManager 实现
//!
//! 统一的引脚管理器，整合 Pinctrl 和 GpioBank，提供简洁易用的引脚配置和 GPIO 操作接口。

use crate::{
    Mmio, PinId,
    pinctrl::{BankId, DriveStrength, Function, PinctrlError, PinctrlResult, Pull},
    variants::rk3588::{gpio::GpioBank, pinctrl::Pinctrl},
};

/// 统一的引脚管理器
///
/// 整合 Pinctrl 和 GpioBank，提供简洁的引脚配置和 GPIO 操作接口。
///
/// # 示例
///
/// ```ignore
/// use rockchip_soc::rk3588::PinManager;
/// use rockchip_soc::pinctrl::{PinId, Pull};
///
/// // 创建 PinManager
/// let manager = unsafe { PinManager::new() };
///
/// // 配置引脚为 GPIO 并设置为输出
/// let pin = PinId::new(0).unwrap();
/// manager.config_gpio_output(pin, Pull::PullUp, true)?;
///
/// // 读取输入引脚
/// let value = manager.read_gpio(pin)?;
/// ```
pub struct PinManager {
    /// Pinctrl 驱动（引脚功能配置）
    pinctrl: Pinctrl,

    /// 5 个 GPIO Bank（GPIO 数据操作）
    gpio_banks: [GpioBank; 5],
}

unsafe impl Send for PinManager {}

impl PinManager {
    /// 创建新的 PinManager
    ///
    /// # Safety
    ///
    /// IOC 和 GPIO 寄存器地址必须有效且在生命周期内保持可访问
    ///
    /// 寄存器地址参考设备树：
    /// - IOC: 0xfd5f0000 (syscon@fd5f0000)
    /// - GPIO0-4: 0xfd8a0000, 0xfec20000, 0xfec30000, 0xfec40000, 0xfec50000
    pub unsafe fn new(ioc: Mmio, gpio: [Mmio; 5]) -> Self {
        Self {
            pinctrl: unsafe { Pinctrl::new(ioc) },
            gpio_banks: [
                GpioBank::new(gpio[0]), // GPIO0 (Pin 0-31)
                GpioBank::new(gpio[1]), // GPIO1 (Pin 32-63)
                GpioBank::new(gpio[2]), // GPIO2 (Pin 64-95)
                GpioBank::new(gpio[3]), // GPIO3 (Pin 96-127)
                GpioBank::new(gpio[4]), // GPIO4 (Pin 128-159)
            ],
        }
    }

    /// 配置引脚为 GPIO 并设置方向为输入
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `pull` - 上下拉配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use rockchip_soc::pinctrl::{PinId, Pull};
    ///
    /// // 配置 GPIO0_A1 为输入，上拉
    /// let pin = PinId::new(1).unwrap();
    /// manager.config_gpio_input(pin, Pull::PullUp)?;
    /// ```
    pub fn config_gpio_input(&self, pin: PinId, pull: Pull) -> PinctrlResult<()> {
        // 1. 配置为 GPIO 功能
        self.pinctrl.set_mux(pin, Function::Gpio)?;

        // 2. 配置上下拉
        self.pinctrl.set_pull(pin, pull)?;

        // 3. 设置为输入方向
        let bank_id = pin.bank().raw() as usize;
        let pin_in_bank = pin.pin_in_bank();
        self.gpio_banks[bank_id]
            .set_direction_input(pin_in_bank)
            .map_err(|e| match e {
                crate::variants::rk3588::gpio::GpioError::InvalidPin(p) => {
                    PinctrlError::InvalidPinId(pin.raw())
                }
                _ => PinctrlError::InvalidConfig,
            })?;

        Ok(())
    }

    /// 配置引脚为 GPIO 并设置方向为输出（初始值）
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `pull` - 上下拉配置
    /// * `value` - 初始输出值
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use rockchip_soc::pinctrl::{PinId, Pull};
    ///
    /// // 配置 GPIO0_A0 为输出，上拉，初始高电平
    /// let pin = PinId::new(0).unwrap();
    /// manager.config_gpio_output(pin, Pull::PullUp, true)?;
    /// ```
    pub fn config_gpio_output(&self, pin: PinId, pull: Pull, value: bool) -> PinctrlResult<()> {
        // 1. 配置为 GPIO 功能
        self.pinctrl.set_mux(pin, Function::Gpio)?;

        // 2. 配置上下拉
        self.pinctrl.set_pull(pin, pull)?;

        // 3. 设置为输出方向并写入初始值
        let bank_id = pin.bank().raw() as usize;
        let pin_in_bank = pin.pin_in_bank();
        self.gpio_banks[bank_id]
            .set_direction_output(pin_in_bank, value)
            .map_err(|e| match e {
                crate::variants::rk3588::gpio::GpioError::InvalidPin(p) => {
                    PinctrlError::InvalidPinId(pin.raw())
                }
                _ => PinctrlError::InvalidConfig,
            })?;

        Ok(())
    }

    /// 读取 GPIO 引脚值
    ///
    /// 引脚必须已配置为 GPIO 功能。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    ///
    /// # 返回
    ///
    /// 引脚电平状态（true = 高电平，false = 低电平）
    pub fn read_gpio(&self, pin: PinId) -> PinctrlResult<bool> {
        let bank_id = pin.bank().raw() as usize;
        let pin_in_bank = pin.pin_in_bank();

        self.gpio_banks[bank_id]
            .read(pin_in_bank)
            .map_err(|e| match e {
                crate::variants::rk3588::gpio::GpioError::InvalidPin(p) => {
                    PinctrlError::InvalidPinId(pin.raw())
                }
                _ => PinctrlError::InvalidConfig,
            })
    }

    /// 写入 GPIO 引脚值
    ///
    /// 引脚必须已配置为 GPIO 输出功能。
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `value` - 输出值（true = 高电平，false = 低电平）
    pub fn write_gpio(&self, pin: PinId, value: bool) -> PinctrlResult<()> {
        let bank_id = pin.bank().raw() as usize;
        let pin_in_bank = pin.pin_in_bank();

        self.gpio_banks[bank_id]
            .write(pin_in_bank, value)
            .map_err(|e| match e {
                crate::variants::rk3588::gpio::GpioError::InvalidPin(p) => {
                    PinctrlError::InvalidPinId(pin.raw())
                }
                _ => PinctrlError::InvalidConfig,
            })
    }

    /// 配置引脚为外设功能（UART/I2C/SPI 等）
    ///
    /// # 参数
    ///
    /// * `pin` - 引脚 ID
    /// * `function` - 外设功能（Alt1-Alt4）
    /// * `pull` - 可选的上下拉配置
    /// * `drive` - 可选的驱动强度配置
    ///
    /// # 示例
    ///
    /// ```ignore
    /// use rockchip_soc::pinctrl::{PinId, Function, Pull, DriveStrength};
    ///
    /// // 配置 UART0 TX (GPIO1_A0 = Pin 32)
    /// let tx_pin = PinId::new(32).unwrap();
    /// manager.config_peripheral(
    ///     tx_pin,
    ///     Function::Alt1,
    ///     Some(Pull::Disabled),
    ///     Some(DriveStrength::Ma8),
    /// )?;
    /// ```
    pub fn config_peripheral(
        &self,
        pin: PinId,
        function: Function,
        pull: Option<Pull>,
        drive: Option<DriveStrength>,
    ) -> PinctrlResult<()> {
        self.pinctrl.set_mux(pin, function)?;

        if let Some(p) = pull {
            self.pinctrl.set_pull(pin, p)?;
        }

        if let Some(d) = drive {
            self.pinctrl.set_drive(pin, d)?;
        }

        Ok(())
    }

    /// 获取底层的 Pinctrl 引用（用于高级配置）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 使用 PinConfig 进行完整配置
    /// use rockchip_soc::pinctrl::PinConfig;
    ///
    /// let config = PinConfig::new(Function::Alt1)
    ///     .with_pull(Pull::PullUp)
    ///     .with_drive(DriveStrength::Ma8);
    ///
    /// manager.pinctrl().config_pin(pin, &config)?;
    /// ```
    pub fn pinctrl(&self) -> &Pinctrl {
        &self.pinctrl
    }

    /// 获取指定的 GPIO Bank 引用（用于直接操作）
    ///
    /// # 参数
    ///
    /// * `bank_id` - GPIO Bank ID (0-4)
    ///
    /// # 返回
    ///
    /// 如果 bank_id 有效，返回 Some(&GpioBank)，否则返回 None
    pub fn gpio_bank(&self, bank_id: BankId) -> Option<&GpioBank> {
        self.gpio_banks.get(bank_id.raw() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 注意：这些测试不会在真实硬件上运行，仅用于编译检查
    // 真正的功能测试需要硬件环境

    #[test]
    fn test_pin_manager_size() {
        // 验证 PinManager 大小合理
        assert_eq!(
            core::mem::size_of::<PinManager>(),
            6 * core::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_send_trait() {
        // 验证 PinManager 实现 Send
        fn assert_send<T: Send>() {}
        assert_send::<PinManager>();
    }
}
