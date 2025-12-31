//! RK3588 时钟门控 (Clock Gate) 管理
//!
//! 参考 Linux: drivers/clk/rockchip/clk-rk3588.c
//!
//! 每个 clkgate_con 寄存器有 32 位，每 bit 控制一个时钟的开关
//! Rockchip 使用写掩码机制：
//! - 高 16 位：要清除的位掩码
//! - 低 16 位：要设置的值
//!
//! 使能时钟：清除对应的 bit
//! 禁止时钟：设置对应的 bit

use super::Cru;
use super::consts::*;
use crate::clock::ClkId;
use crate::rk3588::cru::*;

/// 时钟门控配置
#[derive(Debug, Clone, Copy)]
pub struct ClkGate {
    /// 寄存器索引 (0-31 用于 clkgate_con, 32+ 用于 pmu_clkgate_con)
    pub reg_idx: u32,
    /// 位偏移 (0-15)
    pub bit: u32,
}

// =============================================================================
// 宏定义：批量定义时钟门控常量
// =============================================================================

/// 批量定义时钟门控配置
///
/// # 语法
/// ```ignore
/// clk_gate_group!(
///     PCLK_I2C1 = (10, 8),  // reg_idx=10, bit=8
///     CLK_I2C1 = (11, 0),
/// );
/// ```
macro_rules! clk_gate_group {
    ($($name:ident = ($reg_idx:expr, $bit:expr)),* $(,)?) => {
        $(
            pub const $name: ClkGate = ClkGate {
                reg_idx: $reg_idx,
                bit: $bit,
            };
        )*
    };
}

// =============================================================================
// I2C 时钟门控
// =============================================================================

/// I2C1-8 时钟门控 (参考 Linux: clk-rk3588.c)
pub mod i2c {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_I2C1 = (10, 8),
        CLK_I2C1 = (11, 0),
        PCLK_I2C2 = (10, 9),
        CLK_I2C2 = (11, 1),
        PCLK_I2C3 = (10, 10),
        CLK_I2C3 = (11, 2),
        PCLK_I2C4 = (10, 11),
        CLK_I2C4 = (11, 3),
        PCLK_I2C5 = (10, 12),
        CLK_I2C5 = (11, 4),
        PCLK_I2C6 = (10, 13),
        CLK_I2C6 = (11, 5),
        PCLK_I2C7 = (10, 14),
        CLK_I2C7 = (11, 6),
        PCLK_I2C8 = (10, 15),
        CLK_I2C8 = (11, 7),
    );
}

/// I2C0 (PMU) 时钟门控
pub mod i2c_pmu {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_I2C0 = (0x32 + 2, 1), // PMU_CLKGATE_CON(2), bit 1
        CLK_I2C0 = (0x32 + 2, 2),  // PMU_CLKGATE_CON(2), bit 2
    );
}

// =============================================================================
// SPI 时钟门控
// =============================================================================

/// SPI0-4 时钟门控 (参考 Linux: clk-rk3588.c)
pub mod spi {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_SPI0 = (14, 6),
        CLK_SPI0 = (14, 11),
        PCLK_SPI1 = (14, 7),
        CLK_SPI1 = (14, 12),
        PCLK_SPI2 = (14, 8),
        CLK_SPI2 = (14, 13),
        PCLK_SPI3 = (14, 9),
        CLK_SPI3 = (14, 14),
        PCLK_SPI4 = (14, 10),
        CLK_SPI4 = (14, 15),
    );
}

// =============================================================================
// UART 时钟门控
// =============================================================================

/// UART1-9 时钟门控 (参考 Linux: clk-rk3588.c)
pub mod uart {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_UART1 = (12, 2),
        SCLK_UART1 = (12, 13),
        PCLK_UART2 = (12, 3),
        SCLK_UART2 = (13, 0),
        PCLK_UART3 = (12, 4),
        SCLK_UART3 = (13, 3),
        PCLK_UART4 = (12, 5),
        SCLK_UART4 = (13, 6),
        PCLK_UART5 = (12, 6),
        SCLK_UART5 = (13, 9),
        PCLK_UART6 = (12, 7),
        SCLK_UART6 = (13, 12),
        PCLK_UART7 = (12, 8),
        SCLK_UART7 = (13, 15),
        PCLK_UART8 = (12, 9),
        SCLK_UART8 = (14, 2),
        PCLK_UART9 = (12, 10),
        SCLK_UART9 = (14, 5),
    );
}

/// UART0 (PMU) 时钟门控
pub mod uart_pmu {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_UART0 = (0x32 + 2, 6), // PMU_CLKGATE_CON(2), bit 6
        SCLK_UART0 = (0x32 + 2, 5), // PMU_CLKGATE_CON(2), bit 5
    );
}

// =============================================================================
// PWM 时钟门控
// =============================================================================

/// PWM1-3 时钟门控
pub mod pwm {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_PWM1 = (15, 0),
        CLK_PWM1 = (15, 3),
        CLK_PWM1_CAPTURE = (15, 5),
        PCLK_PWM2 = (15, 6),
        CLK_PWM2 = (15, 7),
        CLK_PWM2_CAPTURE = (15, 8),
        PCLK_PWM3 = (15, 1),
        CLK_PWM3 = (15, 4),
        CLK_PWM3_CAPTURE = (15, 9),
    );
}

/// PMU1PWM 时钟门控
pub mod pwm_pmu {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_PMU1PWM = (0x32 + 2, 8),         // PMU_CLKGATE_CON(2), bit 8
        CLK_PMU1PWM = (0x32 + 2, 11),         // PMU_CLKGATE_CON(2), bit 11
        CLK_PMU1PWM_CAPTURE = (0x32 + 2, 12), // PMU_CLKGATE_CON(2), bit 12
    );
}

// =============================================================================
// ADC 时钟门控
// =============================================================================

/// ADC 时钟门控
pub mod adc {
    use super::ClkGate;

    clk_gate_group!(
        PCLK_SARADC = (15, 11),
        CLK_SARADC = (15, 12),
        PCLK_TSADC = (16, 6),
        CLK_TSADC = (16, 7),
    );
}

// =============================================================================
// 时钟门控映射表
// =============================================================================

/// 时钟门控映射表
///
/// 格式：(时钟ID, 门控配置)
///
/// 参考 Linux kernel: drivers/clk/rockchip/clk-rk3588.c
const CLK_GATE_TABLE: &[(ClkId, ClkGate)] = &[
    // ========================================================================
    // I2C 时钟门控
    // ========================================================================
    (PCLK_I2C1, i2c::PCLK_I2C1),
    (CLK_I2C1, i2c::CLK_I2C1),
    (PCLK_I2C2, i2c::PCLK_I2C2),
    (CLK_I2C2, i2c::CLK_I2C2),
    (PCLK_I2C3, i2c::PCLK_I2C3),
    (CLK_I2C3, i2c::CLK_I2C3),
    (PCLK_I2C4, i2c::PCLK_I2C4),
    (CLK_I2C4, i2c::CLK_I2C4),
    (PCLK_I2C5, i2c::PCLK_I2C5),
    (CLK_I2C5, i2c::CLK_I2C5),
    (PCLK_I2C6, i2c::PCLK_I2C6),
    (CLK_I2C6, i2c::CLK_I2C6),
    (PCLK_I2C7, i2c::PCLK_I2C7),
    (CLK_I2C7, i2c::CLK_I2C7),
    (PCLK_I2C8, i2c::PCLK_I2C8),
    (CLK_I2C8, i2c::CLK_I2C8),
    (PCLK_I2C0, i2c_pmu::PCLK_I2C0),
    (CLK_I2C0, i2c_pmu::CLK_I2C0),
    // ========================================================================
    // SPI 时钟门控
    // ========================================================================
    (PCLK_SPI0, spi::PCLK_SPI0),
    (CLK_SPI0, spi::CLK_SPI0),
    (PCLK_SPI1, spi::PCLK_SPI1),
    (CLK_SPI1, spi::CLK_SPI1),
    (PCLK_SPI2, spi::PCLK_SPI2),
    (CLK_SPI2, spi::CLK_SPI2),
    (PCLK_SPI3, spi::PCLK_SPI3),
    (CLK_SPI3, spi::CLK_SPI3),
    (PCLK_SPI4, spi::PCLK_SPI4),
    (CLK_SPI4, spi::CLK_SPI4),
    // ========================================================================
    // UART 时钟门控
    // ========================================================================
    (PCLK_UART1, uart::PCLK_UART1),
    (SCLK_UART1, uart::SCLK_UART1),
    (PCLK_UART2, uart::PCLK_UART2),
    (SCLK_UART2, uart::SCLK_UART2),
    (PCLK_UART3, uart::PCLK_UART3),
    (SCLK_UART3, uart::SCLK_UART3),
    (PCLK_UART4, uart::PCLK_UART4),
    (SCLK_UART4, uart::SCLK_UART4),
    (PCLK_UART5, uart::PCLK_UART5),
    (SCLK_UART5, uart::SCLK_UART5),
    (PCLK_UART6, uart::PCLK_UART6),
    (SCLK_UART6, uart::SCLK_UART6),
    (PCLK_UART7, uart::PCLK_UART7),
    (SCLK_UART7, uart::SCLK_UART7),
    (PCLK_UART8, uart::PCLK_UART8),
    (SCLK_UART8, uart::SCLK_UART8),
    (PCLK_UART9, uart::PCLK_UART9),
    (SCLK_UART9, uart::SCLK_UART9),
    (PCLK_UART0, uart_pmu::PCLK_UART0),
    (SCLK_UART0, uart_pmu::SCLK_UART0),
    // ========================================================================
    // PWM 时钟门控
    // ========================================================================
    (PCLK_PWM1, pwm::PCLK_PWM1),
    (CLK_PWM1, pwm::CLK_PWM1),
    (CLK_PWM1_CAPTURE, pwm::CLK_PWM1_CAPTURE),
    (PCLK_PWM2, pwm::PCLK_PWM2),
    (CLK_PWM2, pwm::CLK_PWM2),
    (CLK_PWM2_CAPTURE, pwm::CLK_PWM2_CAPTURE),
    (PCLK_PWM3, pwm::PCLK_PWM3),
    (CLK_PWM3, pwm::CLK_PWM3),
    (CLK_PWM3_CAPTURE, pwm::CLK_PWM3_CAPTURE),
    (PCLK_PMU1PWM, pwm_pmu::PCLK_PMU1PWM),
    (CLK_PMU1PWM, pwm_pmu::CLK_PMU1PWM),
    (CLK_PMU1PWM_CAPTURE, pwm_pmu::CLK_PMU1PWM_CAPTURE),
    // ========================================================================
    // ADC 时钟门控
    // ========================================================================
    (PCLK_SARADC, adc::PCLK_SARADC),
    (CLK_SARADC, adc::CLK_SARADC),
    (PCLK_TSADC, adc::PCLK_TSADC),
    (CLK_TSADC, adc::CLK_TSADC),
];

impl Cru {
    /// 查找时钟门控配置
    pub fn find_clk_gate(&self, id: ClkId) -> Option<ClkGate> {
        CLK_GATE_TABLE
            .iter()
            .find(|(clk_id, _)| *clk_id == id)
            .map(|(_, gate)| *gate)
    }

    /// 获取时钟门控寄存器地址
    pub fn get_gate_reg_offset(&self, gate: ClkGate) -> u32 {
        if gate.reg_idx >= 0x32 {
            // PMU CRU: pmu_clkgate_con
            let idx = gate.reg_idx - 0x32;
            pmu_clkgate_con(idx)
        } else {
            // 主 CRU: clkgate_con
            clkgate_con(gate.reg_idx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clk_gate_table_size() {
        // 确保映射表不为空
        assert!(
            !CLK_GATE_TABLE.is_empty(),
            "CLK_GATE_TABLE should not be empty"
        );
    }

    #[test]
    fn test_clk_gate_unique() {
        // 检查是否有重复的 clkid
        let mut clk_ids = CLK_GATE_TABLE
            .iter()
            .map(|(id, _)| id.value())
            .collect::<Vec<_>>();

        clk_ids.sort();
        clk_ids.dedup();

        assert_eq!(
            clk_ids.len(),
            CLK_GATE_TABLE.len(),
            "CLK_GATE_TABLE should not have duplicate clkid entries"
        );
    }

    #[test]
    fn test_i2c_gates() {
        // 验证 I2C gate 配置
        assert_eq!(i2c::PCLK_I2C1.reg_idx, 10);
        assert_eq!(i2c::PCLK_I2C1.bit, 8);
        assert_eq!(i2c::CLK_I2C1.reg_idx, 11);
        assert_eq!(i2c::CLK_I2C1.bit, 0);

        // PMU I2C0
        assert_eq!(i2c_pmu::PCLK_I2C0.reg_idx, 0x32 + 2);
        assert_eq!(i2c_pmu::PCLK_I2C0.bit, 1);
    }

    #[test]
    fn test_spi_gates() {
        // 验证 SPI gate 配置
        assert_eq!(spi::PCLK_SPI0.reg_idx, 14);
        assert_eq!(spi::PCLK_SPI0.bit, 6);
        assert_eq!(spi::CLK_SPI0.reg_idx, 14);
        assert_eq!(spi::CLK_SPI0.bit, 11);
    }

    #[test]
    fn test_uart_gates() {
        // 验证 UART gate 配置
        assert_eq!(uart::PCLK_UART1.reg_idx, 12);
        assert_eq!(uart::PCLK_UART1.bit, 2);
        assert_eq!(uart::SCLK_UART1.reg_idx, 12);
        assert_eq!(uart::SCLK_UART1.bit, 13);

        // PMU UART0
        assert_eq!(uart_pmu::PCLK_UART0.reg_idx, 0x32 + 2);
        assert_eq!(uart_pmu::PCLK_UART0.bit, 6);
    }
}
