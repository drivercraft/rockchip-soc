//! IOMUX 寄存器映射和计算
//!
//! RK3588 的引脚复用配置是规则分布的，使用算法计算而非静态表。

/// IOC 基地址类型
#[derive(Debug, Clone, Copy)]
pub enum IocBase {
    /// PMU1_IOC (0x0000)
    Pmu1,
    /// PMU2_IOC (0x4000)
    Pmu2,
    /// BUS_IOC (0x8000)
    Bus,
    /// VCCIO1-4_IOC (0x9000)
    Vccio14,
    /// VCCIO3-5_IOC (0xA000)
    Vccio35,
    /// VCCIO2_IOC (0xB000)
    Vccio2,
    /// VCCIO6_IOC (0xC000)
    Vccio6,
    /// EMMC_IOC (0xD000)
    Emmc,
}

impl IocBase {
    /// 获取 IOC 基地址偏移
    pub const fn offset(self) -> usize {
        match self {
            Self::Pmu1 => 0x0000,
            Self::Pmu2 => 0x4000,
            Self::Bus => 0x8000,
            Self::Vccio14 => 0x9000,
            Self::Vccio35 => 0xA000,
            Self::Vccio2 => 0xB000,
            Self::Vccio6 => 0xC000,
            Self::Emmc => 0xD000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BankId;

    #[test]
    fn test_ioc_base_offsets() {
        assert_eq!(IocBase::Pmu1.offset(), 0x0000);
        assert_eq!(IocBase::Pmu2.offset(), 0x4000);
        assert_eq!(IocBase::Bus.offset(), 0x8000);
    }

    #[test]
    fn test_gpio0_iomux() {
        // GPIO0_A0 (pin 0)
        let pin = PinId::from_bank_pin(BankId::new(0).unwrap(), 0).unwrap();
        let (config, extra) = calc_iomux_config(pin).unwrap();

        assert_eq!(config.reg_offset, 0x0);
        assert_eq!(config.bit_offset, 0);
        assert!(extra.is_none());

        // GPIO0_A1 (pin 1)
        let pin = PinId::from_bank_pin(BankId::new(0).unwrap(), 1).unwrap();
        let (config, extra) = calc_iomux_config(pin).unwrap();

        assert_eq!(config.reg_offset, 0x0);
        assert_eq!(config.bit_offset, 4);
        assert!(extra.is_none());
    }

    #[test]
    fn test_gpio1_iomux() {
        // GPIO1_A0 (pin 32)
        let pin = PinId::new(32).unwrap();
        let (config, extra) = calc_iomux_config(pin).unwrap();

        // bank 1 使用 BUS_IOC (0x8000)
        assert!(config.reg_offset >= 0x8000);
        assert_eq!(config.bit_offset, 0);
        assert!(extra.is_none());
    }

    #[test]
    fn test_bit_offset_calculation() {
        // 测试不同的位偏移
        for pin_num in 0..4 {
            let pin = PinId::new(pin_num).unwrap();
            let (config, _) = calc_iomux_config(pin).unwrap();
            assert_eq!(config.bit_offset, pin_num * 4);
        }
    }
}
