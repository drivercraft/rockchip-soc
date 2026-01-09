//! RK3588 PinManager 实现
//!
//! 统一的引脚管理器，整合 Pinctrl 和 GpioBank，提供简洁易用的引脚配置和 GPIO 操作接口。

use crate::{
    Mmio, PinConfig, PinId,
    pinctrl::{Iomux, PinctrlResult},
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
    /// IOC 和 GPIO 寄存器地址必须有效且在生命周期内保持可访问
    ///
    /// 寄存器地址参考设备树：
    /// - IOC: 0xfd5f0000 (syscon@fd5f0000)
    /// - GPIO0-4: 0xfd8a0000, 0xfec20000, 0xfec30000, 0xfec40000, 0xfec50000
    pub fn new(ioc: Mmio, gpio: [Mmio; 5]) -> Self {
        let iomux = [Iomux::WIDTH_4BIT; 4];
        Self {
            pinctrl: unsafe { Pinctrl::new(ioc) },
            gpio_banks: [
                GpioBank::new(gpio[0], iomux), // GPIO0 (Pin 0-31)
                GpioBank::new(gpio[1], iomux), // GPIO1 (Pin 32-63)
                GpioBank::new(gpio[2], iomux), // GPIO2 (Pin 64-95)
                GpioBank::new(gpio[3], iomux), // GPIO3 (Pin 96-127)
                GpioBank::new(gpio[4], iomux), // GPIO4 (Pin 128-159)
            ],
        }
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
        self.gpio_banks[bank_id].read(pin)
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
        self.gpio_banks[bank_id].write(pin, value)
    }

    fn bank(&self, pin: PinId) -> &GpioBank {
        &self.gpio_banks[pin.bank().raw() as usize]
    }

    pub fn set_config(&mut self, config: PinConfig) -> PinctrlResult<()> {
        debug!("set_config: {:?}", config);
        self.bank(config.id).verify_mux(config.id, config.mux)?;

        self.pinctrl.set_mux(config.id, config.mux)?;

        self.pinctrl.set_pull(config.id, config.pull)?;

        if let Some(drive) = config.drive {
            self.pinctrl.set_drive(config.id, drive)?;
        }

        Ok(())
    }

    pub fn get_config(&self, pin: PinId) -> PinctrlResult<PinConfig> {
        let function = self.pinctrl.get_mux(pin)?;

        let pull = self.pinctrl.get_pull(pin)?;

        let drive = self.pinctrl.get_drive(pin)?;

        Ok(PinConfig {
            id: pin,
            mux: function,
            pull,
            drive: Some(drive),
        })
    }

    // /// 配置引脚为外设功能（UART/I2C/SPI 等）
    // ///
    // /// # 参数
    // ///
    // /// * `pin` - 引脚 ID
    // /// * `function` - 外设功能（Alt1-Alt4）
    // /// * `pull` - 可选的上下拉配置
    // /// * `drive` - 可选的驱动强度配置
    // ///
    // /// # 示例
    // ///
    // /// ```ignore
    // /// use rockchip_soc::pinctrl::{PinId, Function, Pull, DriveStrength};
    // ///
    // /// // 配置 UART0 TX (GPIO1_A0 = Pin 32)
    // /// let tx_pin = PinId::new(32).unwrap();
    // /// manager.config_peripheral(
    // ///     tx_pin,
    // ///     Function::Alt1,
    // ///     Some(Pull::Disabled),
    // ///     Some(DriveStrength::Ma8),
    // /// )?;
    // /// ```
    // pub fn config_peripheral(&self, config: PinConfig) -> PinctrlResult<()> {
    //     self.pinctrl.set_mux(config.pin_id, config.function)?;

    //     self.pinctrl.set_pull(config.pin_id, config.pull)?;

    //     self.pinctrl.set_drive(config.pin_id, config.drive)?;

    //     if let Function::Gpio(dir) = config.function {
    //         let bank = self
    //             .gpio_bank(config.pin_id.bank())
    //             .ok_or(PinctrlError::InvalidPinId(config.pin_id.raw()))?;
    //         let pin_in_bank = config.pin_id.pin_in_bank();

    //         bank.set_direction(pin_in_bank, dir)
    //             .map_err(|_e| PinctrlError::InvalidConfig)?;
    //     }

    //     Ok(())
    // }

    // /// 获取底层的 Pinctrl 引用（用于高级配置）
    // ///
    // /// # 示例
    // ///
    // /// ```ignore
    // /// // 使用 PinConfig 进行完整配置
    // /// use rockchip_soc::pinctrl::PinConfig;
    // ///
    // /// let config = PinConfig::new(Function::Alt1)
    // ///     .with_pull(Pull::PullUp)
    // ///     .with_drive(DriveStrength::Ma8);
    // ///
    // /// manager.pinctrl().config_pin(pin, &config)?;
    // /// ```
    // pub fn pinctrl(&self) -> &Pinctrl {
    //     &self.pinctrl
    // }

    // /// 获取指定的 GPIO Bank 引用（用于直接操作）
    // ///
    // /// # 参数
    // ///
    // /// * `bank_id` - GPIO Bank ID (0-4)
    // ///
    // /// # 返回
    // ///
    // /// 如果 bank_id 有效，返回 Some(&GpioBank)，否则返回 None
    // pub fn gpio_bank(&self, bank_id: BankId) -> Option<&GpioBank> {
    //     self.gpio_banks.get(bank_id.raw() as usize)
    // }

    // /// 获取引脚的完整配置信息
    // ///
    // /// 读取引脚的所有配置状态，包括功能、上下拉、驱动强度和 GPIO 方向等。
    // /// 用于调试和验证引脚配置是否符合预期。
    // ///
    // /// # 参数
    // ///
    // /// * `pin` - 引脚 ID
    // ///
    // /// # 返回
    // ///
    // /// 返回 `PinConfig` 结构体，包含引脚的完整配置信息。
    // ///
    // /// # 示例
    // ///
    // /// ```ignore
    // /// use rockchip_soc::GPIO3_B6;
    // ///
    // /// // 获取引脚配置
    // /// let config = manager.get_pin_config(GPIO3_B6)?;
    // /// println!("Pin {}: {:?}", config.pin_id, config.gpio_direction);
    // /// ```
    // pub fn get_pin_config(&self, pin: PinId) -> PinctrlResult<PinConfig> {
    //     // 从寄存器读取引脚功能配置
    //     let mut function = self.pinctrl.get_mux(pin)?;

    //     // 从寄存器读取上下拉配置
    //     let pull = self.pinctrl.get_pull(pin)?;

    //     // 从寄存器读取驱动强度配置
    //     let drive = self.pinctrl.get_drive(pin)?;

    //     if let Function::Gpio(dir) = &mut function {
    //         // 如果是 GPIO 功能，获取实际的 GPIO 方向
    //         let bank = self
    //             .gpio_bank(pin.bank())
    //             .ok_or(PinctrlError::InvalidPinId(pin.raw()))?;
    //         let pin_in_bank = pin.pin_in_bank();

    //         let actual_dir = bank
    //             .get_direction(pin_in_bank)
    //             .map_err(|_e| PinctrlError::InvalidConfig)?;

    //         *dir = actual_dir;
    //     }

    //     Ok(PinConfig {
    //         pin_id: pin,
    //         function,
    //         pull,
    //         drive,
    //     })
    // }
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
