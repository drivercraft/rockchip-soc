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
pub(crate) struct ClkGate {
    /// 寄存器索引 (0-31 用于 clkgate_con, 32+ 用于 pmu_clkgate_con)
    pub(crate) reg_idx: u32,
    /// 位偏移 (0-15，Rockchip 的写掩码机制只支持低 16 位)
    pub(crate) bit: u32,
}

/// 时钟门控映射表
///
/// 包含常用外设的时钟门控配置
/// 格式：(时钟ID, (寄存器索引, 位偏移))
const CLK_GATE_TABLE: &[(ClkId, ClkGate)] = &[
    // ========================================================================
    // I2C 时钟门控 (参考 u-boot clk_rk3588.c 和设备树)
    // ========================================================================

    // I2C0 (PMU) - PMU_CRU pmu_clkgate_con[2]
    (
        PCLK_I2C0,
        ClkGate {
            reg_idx: 0x32 + 2,
            bit: 2,
        },
    ),
    (
        CLK_I2C0,
        ClkGate {
            reg_idx: 0x32 + 2,
            bit: 3,
        },
    ),
    // I2C1-8 - CRU clkgate_con[11]
    (
        PCLK_I2C1,
        ClkGate {
            reg_idx: 11,
            bit: 8,
        },
    ),
    (
        CLK_I2C1,
        ClkGate {
            reg_idx: 11,
            bit: 0,
        },
    ),
    (
        PCLK_I2C2,
        ClkGate {
            reg_idx: 11,
            bit: 9,
        },
    ),
    (
        CLK_I2C2,
        ClkGate {
            reg_idx: 11,
            bit: 1,
        },
    ),
    (
        PCLK_I2C3,
        ClkGate {
            reg_idx: 11,
            bit: 10,
        },
    ),
    (
        CLK_I2C3,
        ClkGate {
            reg_idx: 11,
            bit: 2,
        },
    ),
    (
        PCLK_I2C4,
        ClkGate {
            reg_idx: 11,
            bit: 11,
        },
    ),
    (
        CLK_I2C4,
        ClkGate {
            reg_idx: 11,
            bit: 3,
        },
    ),
    (
        PCLK_I2C5,
        ClkGate {
            reg_idx: 11,
            bit: 12,
        },
    ),
    (
        CLK_I2C5,
        ClkGate {
            reg_idx: 11,
            bit: 4,
        },
    ),
    (
        PCLK_I2C6,
        ClkGate {
            reg_idx: 11,
            bit: 13,
        },
    ),
    (
        CLK_I2C6,
        ClkGate {
            reg_idx: 11,
            bit: 5,
        },
    ),
    (
        PCLK_I2C7,
        ClkGate {
            reg_idx: 11,
            bit: 14,
        },
    ),
    (
        CLK_I2C7,
        ClkGate {
            reg_idx: 11,
            bit: 6,
        },
    ),
    (
        PCLK_I2C8,
        ClkGate {
            reg_idx: 11,
            bit: 15,
        },
    ),
    (
        CLK_I2C8,
        ClkGate {
            reg_idx: 11,
            bit: 7,
        },
    ),
    // ========================================================================
    // SPI 时钟门控
    // ========================================================================

    // SPI0-4 - CRU clkgate_con[11]
    (
        PCLK_SPI0,
        ClkGate {
            reg_idx: 11,
            bit: 0,
        },
    ),
    (
        CLK_SPI0,
        ClkGate {
            reg_idx: 11,
            bit: 0,
        },
    ),
    (
        PCLK_SPI1,
        ClkGate {
            reg_idx: 11,
            bit: 1,
        },
    ),
    (
        CLK_SPI1,
        ClkGate {
            reg_idx: 11,
            bit: 1,
        },
    ),
    (
        PCLK_SPI2,
        ClkGate {
            reg_idx: 11,
            bit: 2,
        },
    ),
    (
        CLK_SPI2,
        ClkGate {
            reg_idx: 11,
            bit: 2,
        },
    ),
    (
        PCLK_SPI3,
        ClkGate {
            reg_idx: 11,
            bit: 3,
        },
    ),
    (
        CLK_SPI3,
        ClkGate {
            reg_idx: 11,
            bit: 3,
        },
    ),
    (
        PCLK_SPI4,
        ClkGate {
            reg_idx: 11,
            bit: 4,
        },
    ),
    (
        CLK_SPI4,
        ClkGate {
            reg_idx: 11,
            bit: 4,
        },
    ),
    // ========================================================================
    // UART 时钟门控
    // ========================================================================

    // UART1-9 PCLK - CRU clkgate_con[12]
    (
        PCLK_UART1,
        ClkGate {
            reg_idx: 12,
            bit: 8,
        },
    ),
    (
        PCLK_UART2,
        ClkGate {
            reg_idx: 12,
            bit: 9,
        },
    ),
    (
        PCLK_UART3,
        ClkGate {
            reg_idx: 12,
            bit: 10,
        },
    ),
    (
        PCLK_UART4,
        ClkGate {
            reg_idx: 12,
            bit: 11,
        },
    ),
    (
        PCLK_UART5,
        ClkGate {
            reg_idx: 12,
            bit: 12,
        },
    ),
    (
        PCLK_UART6,
        ClkGate {
            reg_idx: 12,
            bit: 13,
        },
    ),
    (
        PCLK_UART7,
        ClkGate {
            reg_idx: 12,
            bit: 14,
        },
    ),
    (
        PCLK_UART8,
        ClkGate {
            reg_idx: 12,
            bit: 15,
        },
    ),
    (
        PCLK_UART9,
        ClkGate {
            reg_idx: 13,
            bit: 0,
        },
    ),
    // UART0 (PMU) - PMU_CRU pmu_clkgate_con[2]
    (
        PCLK_UART0,
        ClkGate {
            reg_idx: 0x32 + 2,
            bit: 0,
        },
    ),
    (
        CLK_UART0,
        ClkGate {
            reg_idx: 0x32 + 2,
            bit: 0,
        },
    ),
    // ========================================================================
    // PWM 时钟门控
    // ========================================================================

    // PWM1-3 - CRU clkgate_con[12]
    (
        PCLK_PWM1,
        ClkGate {
            reg_idx: 12,
            bit: 0,
        },
    ),
    (
        CLK_PWM1,
        ClkGate {
            reg_idx: 12,
            bit: 0,
        },
    ),
    (
        PCLK_PWM2,
        ClkGate {
            reg_idx: 12,
            bit: 1,
        },
    ),
    (
        CLK_PWM2,
        ClkGate {
            reg_idx: 12,
            bit: 1,
        },
    ),
    (
        PCLK_PWM3,
        ClkGate {
            reg_idx: 12,
            bit: 2,
        },
    ),
    (
        CLK_PWM3,
        ClkGate {
            reg_idx: 12,
            bit: 2,
        },
    ),
    // PMU1PWM - PMU_CRU pmu_clkgate_con[2]
    (
        PCLK_PMU1PWM,
        ClkGate {
            reg_idx: 0x32 + 2,
            bit: 6,
        },
    ),
    (
        CLK_PMU1PWM,
        ClkGate {
            reg_idx: 0x32 + 2,
            bit: 7,
        },
    ),
    // ========================================================================
    // ADC 时钟门控
    // ========================================================================

    // SARADC - CRU clkgate_con[15]
    (
        PCLK_SARADC,
        ClkGate {
            reg_idx: 15,
            bit: 5,
        },
    ),
    (
        CLK_SARADC,
        ClkGate {
            reg_idx: 15,
            bit: 5,
        },
    ),
    // TSADC - CRU clkgate_con[15]
    (
        PCLK_TSADC,
        ClkGate {
            reg_idx: 15,
            bit: 6,
        },
    ),
    (
        CLK_TSADC,
        ClkGate {
            reg_idx: 15,
            bit: 6,
        },
    ),
];

impl Cru {
    /// 查找时钟门控配置
    pub(crate) fn find_clk_gate(&self, id: ClkId) -> Option<ClkGate> {
        CLK_GATE_TABLE
            .iter()
            .find(|(clk_id, _)| *clk_id == id)
            .map(|(_, gate)| *gate)
    }

    /// 获取时钟门控寄存器地址
    pub(crate) fn get_gate_reg_offset(&self, gate: ClkGate) -> u32 {
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

        // 统计不同类型的时钟数量
        let i2c_count = CLK_GATE_TABLE
            .iter()
            .filter(|(id, _)| {
                matches!(id.value(),
                    133..=140 |  // PCLK_I2C1-8
                    141..=148 |  // CLK_I2C1-8
                    646 | 647     // PCLK_I2C0, CLK_I2C0
                )
            })
            .count();

        assert!(i2c_count >= 18, "Should have at least 18 I2C gate entries");
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
}
