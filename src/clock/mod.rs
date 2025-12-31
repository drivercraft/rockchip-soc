use core::ops::RangeBounds;

pub mod pll;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClkId(u64);

impl From<u64> for ClkId {
    fn from(value: u64) -> Self {
        ClkId(value)
    }
}

impl From<usize> for ClkId {
    fn from(value: usize) -> Self {
        ClkId(value as u64)
    }
}

impl From<u32> for ClkId {
    fn from(value: u32) -> Self {
        ClkId(value as u64)
    }
}

impl From<ClkId> for u64 {
    fn from(clk_id: ClkId) -> Self {
        clk_id.0
    }
}

impl core::fmt::Display for ClkId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ClkId({})", self.0)
    }
}

impl ClkId {
    /// 获取时钟 ID 的数值表示
    pub const fn value(&self) -> u64 {
        self.0
    }

    pub const fn new(value: u64) -> Self {
        ClkId(value)
    }
}

impl RangeBounds<ClkId> for ClkId {
    fn start_bound(&self) -> core::ops::Bound<&ClkId> {
        core::ops::Bound::Included(self)
    }

    fn end_bound(&self) -> core::ops::Bound<&ClkId> {
        core::ops::Bound::Included(self)
    }
}

// =============================================================================
// UART 时钟边界常量 (参考 rk3588-cru.h)
// =============================================================================

impl ClkId {
    // UART0 (PMU) 时钟范围: 683-687
    pub const CLK_UART0_SRC: ClkId = ClkId::new(683);
    pub const CLK_UART0_FRAC: ClkId = ClkId::new(684);
    pub const CLK_UART0: ClkId = ClkId::new(685);
    pub const SCLK_UART0: ClkId = ClkId::new(686);
    pub const PCLK_UART0: ClkId = ClkId::new(687);

    // UART1 时钟范围: 171-183
    pub const PCLK_UART1: ClkId = ClkId::new(171);
    pub const SCLK_UART1: ClkId = ClkId::new(183);

    // UART2 时钟范围: 184-187
    pub const CLK_UART2_SRC: ClkId = ClkId::new(184);
    pub const SCLK_UART2: ClkId = ClkId::new(187);

    // UART3 时钟范围: 188-191
    pub const CLK_UART3_SRC: ClkId = ClkId::new(188);
    pub const SCLK_UART3: ClkId = ClkId::new(191);

    // UART4 时钟范围: 192-195
    pub const CLK_UART4_SRC: ClkId = ClkId::new(192);
    pub const SCLK_UART4: ClkId = ClkId::new(195);

    // UART5 时钟范围: 196-199
    pub const CLK_UART5_SRC: ClkId = ClkId::new(196);
    pub const SCLK_UART5: ClkId = ClkId::new(199);

    // UART6 时钟范围: 200-203
    pub const CLK_UART6_SRC: ClkId = ClkId::new(200);
    pub const SCLK_UART6: ClkId = ClkId::new(203);

    // UART7 时钟范围: 204-207
    pub const CLK_UART7_SRC: ClkId = ClkId::new(204);
    pub const SCLK_UART7: ClkId = ClkId::new(207);

    // UART8 时钟范围: 208-211
    pub const CLK_UART8_SRC: ClkId = ClkId::new(208);
    pub const SCLK_UART8: ClkId = ClkId::new(211);

    // UART9 时钟范围: 212-215
    pub const CLK_UART9_SRC: ClkId = ClkId::new(212);
    pub const SCLK_UART9: ClkId = ClkId::new(215);

    // PLL 时钟范围: 1-9
    pub const PLL_B0PLL: ClkId = ClkId::new(1);
    pub const PLL_PPLL: ClkId = ClkId::new(9);

    // I2C 时钟边界
    pub const PCLK_I2C0: ClkId = ClkId::new(646); // PMU I2C0
    pub const CLK_I2C0: ClkId = ClkId::new(647);

    // I2C1-8 单独常量（用于 get_i2c_num）
    pub const PCLK_I2C1: ClkId = ClkId::new(133);
    pub const CLK_I2C1: ClkId = ClkId::new(141);
    pub const PCLK_I2C2: ClkId = ClkId::new(134);
    pub const CLK_I2C2: ClkId = ClkId::new(142);
    pub const PCLK_I2C3: ClkId = ClkId::new(135);
    pub const CLK_I2C3: ClkId = ClkId::new(143);
    pub const PCLK_I2C4: ClkId = ClkId::new(136);
    pub const CLK_I2C4: ClkId = ClkId::new(144);
    pub const PCLK_I2C5: ClkId = ClkId::new(137);
    pub const CLK_I2C5: ClkId = ClkId::new(145);
    pub const PCLK_I2C6: ClkId = ClkId::new(138);
    pub const CLK_I2C6: ClkId = ClkId::new(146);
    pub const PCLK_I2C7: ClkId = ClkId::new(139);
    pub const CLK_I2C7: ClkId = ClkId::new(147);
    pub const PCLK_I2C8: ClkId = ClkId::new(140);
    pub const CLK_I2C8: ClkId = ClkId::new(148);

    // SPI 单独常量（用于 get_spi_num）
    pub const PCLK_SPI0: ClkId = ClkId::new(158);
    pub const CLK_SPI0: ClkId = ClkId::new(163);
    pub const PCLK_SPI1: ClkId = ClkId::new(159);
    pub const CLK_SPI1: ClkId = ClkId::new(164);
    pub const PCLK_SPI2: ClkId = ClkId::new(160);
    pub const CLK_SPI2: ClkId = ClkId::new(165);
    pub const PCLK_SPI3: ClkId = ClkId::new(161);
    pub const CLK_SPI3: ClkId = ClkId::new(166);
    pub const PCLK_SPI4: ClkId = ClkId::new(162);
    pub const CLK_SPI4: ClkId = ClkId::new(167);

    // PWM 时钟边界
    pub const PCLK_PWM1: ClkId = ClkId::new(83); // PWM1-3: 83-91
    pub const CLK_PWM3_CAPTURE: ClkId = ClkId::new(91);
    pub const PCLK_PMU1PWM: ClkId = ClkId::new(676); // PMU PWM: 676-678
    pub const CLK_PMU1PWM_CAPTURE: ClkId = ClkId::new(678);

    // ADC 时钟边界
    pub const PCLK_SARADC: ClkId = ClkId::new(156); // SARADC: 156-157
    pub const CLK_SARADC: ClkId = ClkId::new(157);
    pub const PCLK_TSADC: ClkId = ClkId::new(169); // TSADC: 169-170
    pub const CLK_TSADC: ClkId = ClkId::new(170);
}
