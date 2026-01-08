//! IOMUX 寄存器映射和计算
//!
//! RK3588 的引脚复用配置是规则分布的，使用算法计算而非静态表。

use crate::PinId;

/// iomux 寄存器配置
#[derive(Debug, Clone, Copy)]
pub struct IomuxConfig {
    /// 寄存器绝对地址（相对 IOC 基地址的偏移）
    pub reg_offset: usize,

    /// 位偏移（每个引脚占 4 位）
    pub bit_offset: u32,

    /// 是否需要特殊的双寄存器配置（GPIO0 的某些引脚）
    pub dual_register: bool,
}

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

/// 计算引脚的 iomux 配置
///
/// # 参数
///
/// * `pin` - 引脚 ID
///
/// # 返回
///
/// 返回 `Some(IomuxConfig)` 如果引脚有效，否则返回 `None`
pub fn calc_iomux_config(pin: PinId) -> Option<(IomuxConfig, Option<IomuxConfig>)> {
    let bank = pin.bank().raw() as u32;
    let pin_in_bank = pin.pin_in_bank();

    // 每组 8 个引脚，每组 2 个寄存器（每个寄存器 4 个引脚）
    let iomux_num = pin_in_bank / 8;
    let reg_in_group = if (pin_in_bank % 8) >= 4 { 0x4 } else { 0x0 };
    let bit_offset = (pin_in_bank % 4) * 4;

    // 基础寄存器偏移（相对 PMU1_IOC）
    let base_reg_offset = (iomux_num * 8) as usize + reg_in_group as usize;

    // GPIO0 (bank 0) 的特殊处理
    if bank == 0 {
        // GPIO0_PB4 (pin 12) 到 GPIO0_PD7 (pin 31) 需要双寄存器配置
        if pin_in_bank >= 12 && pin_in_bank <= 31 {
            let pmu2_offset = base_reg_offset + IocBase::Pmu2.offset();
            let bus_offset = base_reg_offset + IocBase::Bus.offset();

            let pmu2_config = IomuxConfig {
                reg_offset: pmu2_offset - 0xC, // 调整偏移
                bit_offset,
                dual_register: false,
            };

            let bus_config = IomuxConfig {
                reg_offset: bus_offset,
                bit_offset,
                dual_register: false,
            };

            return Some((pmu2_config, Some(bus_config)));
        }

        // GPIO0_PA0 到 GPIO0_PB3 使用 PMU1_IOC
        let config = IomuxConfig {
            reg_offset: base_reg_offset,
            bit_offset,
            dual_register: false,
        };

        return Some((config, None));
    }

    // GPIO1-4 (bank 1-4) 使用 BUS_IOC
    let bus_offset = base_reg_offset + IocBase::Bus.offset();
    let config = IomuxConfig {
        reg_offset: bus_offset,
        bit_offset,
        dual_register: false,
    };

    Some((config, None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinctrl::BankId;

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
