//! RK3588 时钟 ID 定义
//!
//! 参考 u-boot: include/dt-bindings/clock/rk3588-cru.h
//!
//! 所有 clkid 值与 Linux/u-boot 定义严格一致，不可随意修改

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
    PCLK_I2C1 = 133,
    PCLK_I2C2 = 134,
    PCLK_I2C3 = 135,
    PCLK_I2C4 = 136,
    PCLK_I2C5 = 137,
    PCLK_I2C6 = 138,
    PCLK_I2C7 = 139,
    PCLK_I2C8 = 140,
);

clk_id_group!(
    CLK_I2C1 = 141,
    CLK_I2C2 = 142,
    CLK_I2C3 = 143,
    CLK_I2C4 = 144,
    CLK_I2C5 = 145,
    CLK_I2C6 = 146,
    CLK_I2C7 = 147,
    CLK_I2C8 = 148,
);

// PMU I2C (I2C0) 在 PMU CRU 中
clk_id_group!(PCLK_I2C0 = 646, CLK_I2C0 = 647,);

// =============================================================================
// UART 时钟 ID
// =============================================================================

clk_id_group!(
    PCLK_UART1 = 171,
    PCLK_UART2 = 172,
    PCLK_UART3 = 173,
    PCLK_UART4 = 174,
    PCLK_UART5 = 175,
    PCLK_UART6 = 176,
    PCLK_UART7 = 177,
    PCLK_UART8 = 178,
    PCLK_UART9 = 179,
);

clk_id_group!(
    CLK_UART1_SRC = 180,
    CLK_UART1_FRAC = 181,
    CLK_UART1 = 182,
    SCLK_UART1 = 183,
    CLK_UART2_SRC = 184,
    CLK_UART2_FRAC = 185,
    CLK_UART2 = 186,
    SCLK_UART2 = 187,
    CLK_UART3_SRC = 188,
    CLK_UART3_FRAC = 189,
    CLK_UART3 = 190,
    SCLK_UART3 = 191,
    CLK_UART4_SRC = 192,
    CLK_UART4_FRAC = 193,
    CLK_UART4 = 194,
    SCLK_UART4 = 195,
    CLK_UART5_SRC = 196,
    CLK_UART5_FRAC = 197,
    CLK_UART5 = 198,
    SCLK_UART5 = 199,
    CLK_UART6_SRC = 200,
    CLK_UART6_FRAC = 201,
    CLK_UART6 = 202,
    SCLK_UART6 = 203,
    CLK_UART7_SRC = 204,
    CLK_UART7_FRAC = 205,
    CLK_UART7 = 206,
    SCLK_UART7 = 207,
    CLK_UART8_SRC = 208,
    CLK_UART8_FRAC = 209,
    CLK_UART8 = 210,
    SCLK_UART8 = 211,
    CLK_UART9_SRC = 212,
    CLK_UART9_FRAC = 213,
    CLK_UART9 = 214,
    SCLK_UART9 = 215,
);

// PMU UART (UART0) 在 PMU CRU 中
clk_id_group!(
    CLK_UART0_SRC = 683,
    CLK_UART0_FRAC = 684,
    CLK_UART0 = 685,
    SCLK_UART0 = 686,
    PCLK_UART0 = 687,
);

// =============================================================================
// SPI 时钟 ID
// =============================================================================

clk_id_group!(
    PCLK_SPI0 = 158,
    PCLK_SPI1 = 159,
    PCLK_SPI2 = 160,
    PCLK_SPI3 = 161,
    PCLK_SPI4 = 162,
);

clk_id_group!(
    CLK_SPI0 = 163,
    CLK_SPI1 = 164,
    CLK_SPI2 = 165,
    CLK_SPI3 = 166,
    CLK_SPI4 = 167,
);

// =============================================================================
// PWM 时钟 ID
// =============================================================================

clk_id_group!(
    PCLK_PWM1 = 83,
    CLK_PWM1 = 84,
    CLK_PWM1_CAPTURE = 85,
    PCLK_PWM2 = 86,
    CLK_PWM2 = 87,
    CLK_PWM2_CAPTURE = 88,
    PCLK_PWM3 = 89,
    CLK_PWM3 = 90,
    CLK_PWM3_CAPTURE = 91,
);

// PMU PWM 在 PMU CRU 中
clk_id_group!(
    PCLK_PMU1PWM = 676,
    CLK_PMU1PWM = 677,
    CLK_PMU1PWM_CAPTURE = 678,
);

// =============================================================================
// ADC 时钟 ID
// =============================================================================

clk_id_group!(PCLK_SARADC = 156, CLK_SARADC = 157,);

clk_id_group!(PCLK_TSADC = 169, CLK_TSADC = 170,);

// =============================================================================
// 根时钟 ID
// =============================================================================

clk_id_group!(
    ACLK_BUS_ROOT = 123,
    ACLK_TOP_ROOT = 270,
    PCLK_TOP_ROOT = 271,
    ACLK_LOW_TOP_ROOT = 272,
    ACLK_CENTER_ROOT = 216,
    ACLK_CENTER_LOW_ROOT = 217,
    HCLK_CENTER_ROOT = 218,
    PCLK_CENTER_ROOT = 219,
);

// =============================================================================
// SDMMC/EMMC/SFC 时钟 ID
// =============================================================================

clk_id_group!(
    HCLK_SDIO = 409,
    CCLK_SRC_SDIO = 410,
    HCLK_EMMC = 312,
    ACLK_EMMC = 313,
    CCLK_EMMC = 314,
    BCLK_EMMC = 315,
    TMCLK_EMMC = 316,
    SCLK_SFC = 317,
    HCLK_SFC = 318,
    HCLK_SFC_XIP = 319,
);

// =============================================================================
// GMAC 时钟 ID
// =============================================================================

clk_id_group!(
    CLK_GMAC0_PTP_REF = 322,
    CLK_GMAC1_PTP_REF = 323,
    CLK_GMAC_125M = 324,
    CLK_GMAC_50M = 325,
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
    // CLK_I2C0 (647), CLK_I2C1-8 (141-148)
    // PCLK_I2C0 (646), PCLK_I2C1-8 (133-140)
    matches!(id, 141..=148 | 647 | 133..=140 | 646)
}

/// 判断时钟 ID 是否为 UART
#[must_use]
pub const fn is_uart_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    // CLK_UART1-9 (182, 186, 190, 194, 198, 202, 206, 210, 214)
    // CLK_UART0 (685)
    // SCLK_UART0-9 (686, 183, 187, 191, 195, 199, 203, 207, 211, 215)
    // PCLK_UART0-9 (687, 171-179)
    // CLK_UARTx_SRC, CLK_UARTx_FRAC (180-181, 184-185, etc.)
    matches!(id,
        171..=182 | 183..=186 | 187..=190 | 191..=194 | 195..=198 |
        199..=202 | 203..=206 | 207..=210 | 211..=215 |
        683..=687
    )
}

/// 判断时钟 ID 是否为 SPI
#[must_use]
pub const fn is_spi_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    matches!(id, 158..=167)
}

/// 判断时钟 ID 是否为 PWM
#[must_use]
pub const fn is_pwm_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    matches!(id,
        83..=91 |  // PWM1-3
        676..=678   // PMU1PWM
    )
}

/// 判断时钟 ID 是否为 ADC
#[must_use]
pub const fn is_adc_clk(clk_id: ClkId) -> bool {
    let id = clk_id.value();
    matches!(id, 156..=157 | 169..=170)
}

/// 获取 I2C 编号 (0-8)
///
/// # 返回
///
/// 返回 I2C 编号，如果不是 I2C 时钟则返回 None
#[must_use]
pub const fn get_i2c_num(clk_id: ClkId) -> Option<u32> {
    let id = clk_id.value();
    match id {
        646 | 647 => Some(0), // I2C0 (PMU)
        141 | 133 => Some(1), // I2C1
        142 | 134 => Some(2), // I2C2
        143 | 135 => Some(3), // I2C3
        144 | 136 => Some(4), // I2C4
        145 | 137 => Some(5), // I2C5
        146 | 138 => Some(6), // I2C6
        147 | 139 => Some(7), // I2C7
        148 | 140 => Some(8), // I2C8
        _ => None,
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
    match id {
        683..=687 => Some(0), // UART0 (PMU)
        171..=183 => Some(1), // UART1
        184..=187 => Some(2), // UART2
        188..=191 => Some(3), // UART3
        192..=195 => Some(4), // UART4
        196..=199 => Some(5), // UART5
        200..=203 => Some(6), // UART6
        204..=207 => Some(7), // UART7
        208..=211 => Some(8), // UART8
        212..=215 => Some(9), // UART9
        _ => None,
    }
}

/// 获取 SPI 编号 (0-4)
///
/// # 返回
///
/// 返回 SPI 编号，如果不是 SPI 时钟则返回 None
#[must_use]
pub const fn get_spi_num(clk_id: ClkId) -> Option<u32> {
    let id = clk_id.value();
    match id {
        158 | 163 => Some(0), // SPI0
        159 | 164 => Some(1), // SPI1
        160 | 165 => Some(2), // SPI2
        161 | 166 => Some(3), // SPI3
        162 | 167 => Some(4), // SPI4
        _ => None,
    }
}

// =============================================================================
// 单元测试
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clk_id_values_match_uboot() {
        // PLL
        assert_eq!(PLL_GPLL.value(), 7);
        assert_eq!(PLL_CPLL.value(), 6);

        // I2C
        assert_eq!(CLK_I2C0.value(), 647, "CLK_I2C0 should match u-boot (647)");
        assert_eq!(CLK_I2C1.value(), 141, "CLK_I2C1 should match u-boot (141)");
        assert_eq!(
            PCLK_I2C0.value(),
            646,
            "PCLK_I2C0 should match u-boot (646)"
        );
        assert_eq!(
            PCLK_I2C1.value(),
            133,
            "PCLK_I2C1 should match u-boot (133)"
        );

        // UART
        assert_eq!(
            CLK_UART0.value(),
            685,
            "CLK_UART0 should match u-boot (685)"
        );
        assert_eq!(
            CLK_UART1.value(),
            182,
            "CLK_UART1 should match u-boot (182)"
        );
        assert_eq!(
            PCLK_UART0.value(),
            687,
            "PCLK_UART0 should match u-boot (687)"
        );
        assert_eq!(
            PCLK_UART1.value(),
            171,
            "PCLK_UART1 should match u-boot (171)"
        );
        assert_eq!(
            SCLK_UART0.value(),
            686,
            "SCLK_UART0 should match u-boot (686)"
        );
        assert_eq!(
            SCLK_UART1.value(),
            183,
            "SCLK_UART1 should match u-boot (183)"
        );

        // SPI
        assert_eq!(CLK_SPI0.value(), 163, "CLK_SPI0 should match u-boot (163)");
        assert_eq!(
            PCLK_SPI0.value(),
            158,
            "PCLK_SPI0 should match u-boot (158)"
        );

        // PWM
        assert_eq!(CLK_PWM1.value(), 84, "CLK_PWM1 should match u-boot (84)");
        assert_eq!(
            CLK_PMU1PWM.value(),
            677,
            "CLK_PMU1PWM should match u-boot (677)"
        );

        // ADC
        assert_eq!(
            CLK_SARADC.value(),
            157,
            "CLK_SARADC should match u-boot (157)"
        );
        assert_eq!(
            CLK_TSADC.value(),
            170,
            "CLK_TSADC should match u-boot (170)"
        );
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
        assert!(is_i2c_clk(PCLK_I2C0));
        assert!(!is_i2c_clk(CLK_UART0));
    }

    #[test]
    fn test_is_uart_clk() {
        assert!(is_uart_clk(CLK_UART0));
        assert!(is_uart_clk(CLK_UART9));
        assert!(is_uart_clk(PCLK_UART1));
        assert!(is_uart_clk(SCLK_UART4));
        assert!(!is_uart_clk(CLK_I2C1));
    }

    #[test]
    fn test_is_spi_clk() {
        assert!(is_spi_clk(CLK_SPI0));
        assert!(is_spi_clk(PCLK_SPI4));
        assert!(!is_spi_clk(CLK_UART0));
    }

    #[test]
    fn test_is_pwm_clk() {
        assert!(is_pwm_clk(CLK_PWM1));
        assert!(is_pwm_clk(CLK_PMU1PWM));
        assert!(!is_pwm_clk(CLK_UART0));
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

    #[test]
    fn test_get_spi_num() {
        assert_eq!(get_spi_num(CLK_SPI0), Some(0));
        assert_eq!(get_spi_num(CLK_SPI4), Some(4));
        assert_eq!(get_spi_num(PCLK_SPI2), Some(2));
        assert_eq!(get_spi_num(CLK_UART0), None);
    }
}
