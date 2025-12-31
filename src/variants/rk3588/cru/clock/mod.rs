//! RK3588 时钟 ID 定义
//!
//! 参考 u-boot: include/dt-bindings/clock/rk3588-cru.h

#![allow(dead_code)]

use crate::clock::ClkId;

// =============================================================================
// 宏定义：批量定义时钟常量
// =============================================================================

/// 批量定义时钟 ID 常量
///
/// # 语法
/// ```ignore
/// clk_id_group!(
///     PLL_B0PLL = 1,
///     PLL_B1PLL = 2,
/// );
/// ```
macro_rules! clk_id_group {
    ($($name:ident = $value:expr),* $(,)?) => {
        $(
            pub const $name: ClkId = ClkId::new($value);
        )*
    };
}

// =============================================================================
// PLL 时钟 ID
// =============================================================================

clk_id_group!(
    PLL_B0PLL = 1,
    PLL_B1PLL = 2,
    PLL_LPLL = 3,
    PLL_V0PLL = 4,
    PLL_AUPLL = 5,
    PLL_CPLL = 6,
    PLL_GPLL = 7,
    PLL_NPLL = 8,
    PLL_PPLL = 9,
);

// =============================================================================
// I2C 时钟 ID
// =============================================================================

clk_id_group!(
    CLK_I2C0 = 146,
    CLK_I2C1 = 147,
    CLK_I2C2 = 148,
    CLK_I2C3 = 149,
    CLK_I2C4 = 150,
    CLK_I2C5 = 151,
    CLK_I2C6 = 152,
    CLK_I2C7 = 153,
    CLK_I2C8 = 154,
);

clk_id_group!(
    PCLK_I2C0 = 620,
    PCLK_I2C1 = 133,
    PCLK_I2C2 = 134,
    PCLK_I2C3 = 135,
    PCLK_I2C4 = 136,
    PCLK_I2C5 = 137,
    PCLK_I2C6 = 138,
    PCLK_I2C7 = 139,
    PCLK_I2C8 = 140,
);

// =============================================================================
// UART 时钟 ID
// =============================================================================

clk_id_group!(
    CLK_UART0 = 622,
    CLK_UART1 = 623,
    CLK_UART2 = 624,
    CLK_UART3 = 625,
    CLK_UART4 = 626,
    CLK_UART5 = 627,
    CLK_UART6 = 628,
    CLK_UART7 = 629,
    CLK_UART8 = 630,
    CLK_UART9 = 631,
);

clk_id_group!(
    PCLK_UART0 = 612,
    PCLK_UART1 = 613,
    PCLK_UART2 = 614,
    PCLK_UART3 = 615,
    PCLK_UART4 = 616,
    PCLK_UART5 = 617,
    PCLK_UART6 = 618,
    PCLK_UART7 = 619,
    PCLK_UART8 = 620,
    PCLK_UART9 = 621,
);

clk_id_group!(
    SCLK_UART0 = 632,
    SCLK_UART1 = 633,
    SCLK_UART2 = 634,
    SCLK_UART3 = 635,
    SCLK_UART4 = 636,
);

// =============================================================================
// SPI 时钟 ID
// =============================================================================

clk_id_group!(
    CLK_SPI0 = 165,
    CLK_SPI1 = 166,
    CLK_SPI2 = 167,
    CLK_SPI3 = 168,
    CLK_SPI4 = 169,
);

// =============================================================================
// PWM 时钟 ID
// =============================================================================

clk_id_group!(
    CLK_PWM1 = 84,
    CLK_PWM2 = 87,
    CLK_PWM3 = 90,
    CLK_PMU1PWM = 646,
);

clk_id_group!(PCLK_PWM1 = 83, PCLK_PWM2 = 86, PCLK_PWM3 = 89,);

// =============================================================================
// ADC 时钟 ID
// =============================================================================

clk_id_group!(CLK_SARADC = 653, CLK_TSADC = 654,);

// =============================================================================
// 根时钟 ID
// =============================================================================

clk_id_group!(
    ACLK_BUS_ROOT = 123,
    ACLK_TOP_ROOT = 652,
    PCLK_TOP_ROOT = 651,
    ACLK_LOW_TOP_ROOT = 650,
    ACLK_CENTER_ROOT = 649,
    PCLK_CENTER_ROOT = 644,
    HCLK_CENTER_ROOT = 645,
    ACLK_CENTER_LOW_ROOT = 643,
);

// 内部使用的虚拟时钟 ID（非实际硬件时钟）
clk_id_group!(ACLK_TOP_S400 = 0, ACLK_TOP_S200 = 0,);

// =============================================================================
// SDMMC/EMMC 时钟 ID
// =============================================================================

clk_id_group!(
    CCLK_SRC_SDIO = 661,
    CCLK_EMMC = 660,
    BCLK_EMMC = 662,
    SCLK_SFC = 663,
);

// =============================================================================
// 辅助函数：时钟类型判断和外设编号提取
// =============================================================================

/// 判断时钟 ID 是否为 PLL
#[must_use]
pub const fn is_pll_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    id >= 1 && id <= 10
}

/// 判断时钟 ID 是否为 I2C
#[must_use]
pub const fn is_i2c_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    (id >= 146 && id <= 154) || (id >= 133 && id <= 140)
}

/// 判断时钟 ID 是否为 UART
#[must_use]
pub const fn is_uart_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    (id >= 622 && id <= 631) || (id >= 612 && id <= 621) || (id >= 632 && id <= 636)
}

/// 判断时钟 ID 是否为 SPI
#[must_use]
pub const fn is_spi_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    id >= 165 && id <= 169
}

/// 获取 I2C 编号 (0-8)
///
/// # 返回
///
/// 返回 I2C 编号，如果不是 I2C 时钟则返回 None
#[must_use]
pub const fn get_i2c_num(clk_id: ClkId) -> Option<u32> {
    let id = clk_id.value();
    if id >= 146 && id <= 154 {
        Some((id - 146) as u32)
    } else if id >= 133 && id <= 140 {
        Some((id - 133) as u32)
    } else if id == 620 {
        Some(0) // PCLK_I2C0
    } else {
        None
    }
}

/// 获取 UART 编号 (0-9)
///
/// # 返回
///
/// 返回 UART 编号，如果不是 UART 时钟则返回 None
#[must_use]
pub const fn get_uart_num(clk_id: ClkId) -> Option<u32> {
    let id = clk_id.value();
    if id >= 622 && id <= 631 {
        Some((id - 622) as u32)
    } else if id >= 612 && id <= 621 {
        Some((id - 612) as u32)
    } else if id >= 632 && id <= 636 {
        Some((id - 632) as u32)
    } else {
        None
    }
}

// =============================================================================
// 单元测试
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clk_id_values() {
        assert_eq!(PLL_GPLL.value(), 7);
        assert_eq!(PLL_CPLL.value(), 6);
        assert_eq!(CLK_I2C1.value(), 147);
        assert_eq!(CLK_UART0.value(), 622);
    }

    #[test]
    fn test_is_pll_clk() {
        assert!(is_pll_clk(PLL_GPLL));
        assert!(is_pll_clk(PLL_CPLL));
        assert!(!is_pll_clk(CLK_I2C1));
    }

    #[test]
    fn test_is_i2c_clk() {
        assert!(is_i2c_clk(CLK_I2C1));
        assert!(is_i2c_clk(CLK_I2C8));
        assert!(!is_i2c_clk(CLK_UART0));
    }

    #[test]
    fn test_is_uart_clk() {
        assert!(is_uart_clk(CLK_UART0));
        assert!(is_uart_clk(CLK_UART9));
        assert!(!is_uart_clk(CLK_I2C1));
    }

    #[test]
    fn test_get_i2c_num() {
        assert_eq!(get_i2c_num(CLK_I2C1), Some(1));
        assert_eq!(get_i2c_num(CLK_I2C8), Some(8));
        assert_eq!(get_i2c_num(PCLK_I2C0), Some(0));
        assert_eq!(get_i2c_num(CLK_UART0), None);
    }

    #[test]
    fn test_get_uart_num() {
        assert_eq!(get_uart_num(CLK_UART0), Some(0));
        assert_eq!(get_uart_num(CLK_UART9), Some(9));
        assert_eq!(get_uart_num(PCLK_UART1), Some(1));
        assert_eq!(get_uart_num(SCLK_UART4), Some(4));
        assert_eq!(get_uart_num(CLK_I2C1), None);
    }
}
