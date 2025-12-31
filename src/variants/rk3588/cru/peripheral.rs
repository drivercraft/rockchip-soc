//! RK3588 外设时钟配置
//!
//! 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c

use super::Cru;
use crate::{clock::ClkId, rk3588::cru::clock::*, rk3588::cru::consts::*};

impl Cru {
    // ========================================================================
    // I2C 时钟
    // ========================================================================

    /// 获取 I2C 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_i2c_get_clk()
    ///
    /// I2C 时钟源选择：100MHz 或 200MHz
    pub(crate) fn i2c_get_rate(&self, id: ClkId) -> u64 {
        let (con, sel_shift) = match id {
            CLK_I2C0 => (pmu_clksel_con(3), 6),
            CLK_I2C1 => (clksel_con(38), 6),
            CLK_I2C2 => (clksel_con(38), 7),
            CLK_I2C3 => (clksel_con(38), 8),
            CLK_I2C4 => (clksel_con(38), 9),
            CLK_I2C5 => (clksel_con(38), 10),
            CLK_I2C6 => (clksel_con(38), 11),
            CLK_I2C7 => (clksel_con(38), 12),
            CLK_I2C8 => (clksel_con(38), 13),
            _ => return 0,
        };

        let sel = (self.read(con) >> sel_shift) & 1;
        if sel == 0 { 200 * MHZ } else { 100 * MHZ }
    }

    /// 设置 I2C 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_i2c_set_clk()
    ///
    /// # 时钟源
    ///
    /// - 100MHz: GPLL/12 或 CPLL/15
    /// - 200MHz: GPLL/6 或 CPLL/7.5
    pub(crate) fn i2c_set_rate(&mut self, id: ClkId, rate_hz: u64) -> u64 {
        let src_200m = if rate_hz >= 198 * MHZ { 0 } else { 1 };

        let (offset, mask, shift) = match id {
            CLK_I2C0 => (pmu_clksel_con(3), 1 << 6, 6),
            CLK_I2C1 => (clksel_con(38), 1 << 6, 6),
            CLK_I2C2 => (clksel_con(38), 1 << 7, 7),
            CLK_I2C3 => (clksel_con(38), 1 << 8, 8),
            CLK_I2C4 => (clksel_con(38), 1 << 9, 9),
            CLK_I2C5 => (clksel_con(38), 1 << 10, 10),
            CLK_I2C6 => (clksel_con(38), 1 << 11, 11),
            CLK_I2C7 => (clksel_con(38), 1 << 12, 12),
            CLK_I2C8 => (clksel_con(38), 1 << 13, 13),
            _ => return 0,
        };

        self.clrsetreg(offset, mask, src_200m << shift);

        if src_200m == 0 { 200 * MHZ } else { 100 * MHZ }
    }

    // ========================================================================
    // SPI 时钟
    // ========================================================================

    /// 获取 SPI 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_spi_get_clk()
    pub(crate) fn spi_get_rate(&self, id: ClkId) -> u64 {
        let con = self.read(clksel_con(59));
        let sel_shift = match id {
            CLK_SPI0 => 2,
            CLK_SPI1 => 4,
            CLK_SPI2 => 6,
            CLK_SPI3 => 8,
            CLK_SPI4 => 10,
            _ => return 0,
        };

        let sel = (con >> sel_shift) & 0x3;
        match sel {
            0 => 200 * MHZ, // CLK_SPI_SEL_200M
            1 => 150 * MHZ, // CLK_SPI_SEL_150M
            2 => OSC_HZ,    // CLK_SPI_SEL_24M
            _ => 0,
        }
    }

    /// 设置 SPI 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_spi_set_clk()
    pub(crate) fn spi_set_rate(&mut self, id: ClkId, rate_hz: u64) -> u64 {
        let src_clk = if rate_hz >= 198 * MHZ {
            0 // CLK_SPI_SEL_200M
        } else if rate_hz >= 140 * MHZ {
            1 // CLK_SPI_SEL_150M
        } else {
            2 // CLK_SPI_SEL_24M
        };

        let (mask, shift) = match id {
            CLK_SPI0 => (0x3 << 2, 2),
            CLK_SPI1 => (0x3 << 4, 4),
            CLK_SPI2 => (0x3 << 6, 6),
            CLK_SPI3 => (0x3 << 8, 8),
            CLK_SPI4 => (0x3 << 10, 10),
            _ => return 0,
        };

        self.clrsetreg(clksel_con(59), mask, src_clk << shift);

        match src_clk {
            0 => 200 * MHZ,
            1 => 150 * MHZ,
            2 => OSC_HZ,
            _ => 0,
        }
    }

    // ========================================================================
    // PWM 时钟
    // ========================================================================

    /// 获取 PWM 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_pwm_get_clk()
    pub(crate) fn pwm_get_rate(&self, id: ClkId) -> u64 {
        let (con, sel_shift) = match id {
            CLK_PWM1 => (clksel_con(59), 12),
            CLK_PWM2 => (clksel_con(59), 14),
            CLK_PWM3 => (clksel_con(60), 0),
            CLK_PMU1PWM => (pmu_clksel_con(2), 9),
            _ => return 0,
        };

        let sel = (self.read(con) >> sel_shift) & 0x3;
        match sel {
            0 => 100 * MHZ, // CLK_PWM_SEL_100M
            1 => 50 * MHZ,  // CLK_PWM_SEL_50M
            2 => OSC_HZ,    // CLK_PWM_SEL_24M
            _ => 0,
        }
    }

    /// 设置 PWM 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_pwm_set_clk()
    pub(crate) fn pwm_set_rate(&mut self, id: ClkId, rate_hz: u64) -> u64 {
        let src_clk = if rate_hz >= 99 * MHZ {
            0 // CLK_PWM_SEL_100M
        } else if rate_hz >= 50 * MHZ {
            1 // CLK_PWM_SEL_50M
        } else {
            2 // CLK_PWM_SEL_24M
        };

        let (offset, mask, shift) = match id {
            CLK_PWM1 => (clksel_con(59), 0x3 << 12, 12),
            CLK_PWM2 => (clksel_con(59), 0x3 << 14, 14),
            CLK_PWM3 => (clksel_con(60), 0x3, 0),
            CLK_PMU1PWM => (pmu_clksel_con(2), 0x3 << 9, 9),
            _ => return 0,
        };

        self.clrsetreg(offset, mask, src_clk << shift);

        match src_clk {
            0 => 100 * MHZ,
            1 => 50 * MHZ,
            2 => OSC_HZ,
            _ => 0,
        }
    }

    // ========================================================================
    // ADC (SARADC/TSADC) 时钟
    // ========================================================================

    /// 获取 ADC 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_adc_get_clk()
    pub(crate) fn adc_get_rate(&self, id: ClkId) -> u64 {
        match id {
            CLK_SARADC => {
                let con = self.read(clksel_con(40));
                let div = ((con & 0xFF) >> 6) as u64;
                let sel = (con >> 14) & 1;
                let prate = if sel == 1 { OSC_HZ } else { self.gpll_hz };
                prate / (div + 1)
            }
            CLK_TSADC => {
                let con = self.read(clksel_con(41));
                let div = (con & 0xFF) as u64;
                let sel = (con >> 8) & 1;
                let prate = if sel == 1 { OSC_HZ } else { 100 * MHZ };
                prate / (div + 1)
            }
            _ => 0,
        }
    }

    /// 设置 ADC 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_adc_set_clk()
    pub(crate) fn adc_set_rate(&mut self, id: ClkId, rate_hz: u64) -> u64 {
        match id {
            CLK_SARADC => {
                if OSC_HZ.is_multiple_of(rate_hz) {
                    let src_clk_div = (OSC_HZ / rate_hz) as u32;
                    self.clrsetreg(
                        clksel_con(40),
                        (1 << 14) | (0xFF << 6),
                        (1 << 14) | ((src_clk_div - 1) << 6),
                    );
                    OSC_HZ / (src_clk_div as u64)
                } else {
                    let src_clk_div = (self.gpll_hz / rate_hz) as u32;
                    self.clrsetreg(
                        clksel_con(40),
                        (1 << 14) | (0xFF << 6),
                        (((src_clk_div - 1) << 6)),
                    );
                    self.gpll_hz / (src_clk_div as u64)
                }
            }
            CLK_TSADC => {
                if OSC_HZ.is_multiple_of(rate_hz) {
                    let src_clk_div = (OSC_HZ / rate_hz).min(255) as u32;
                    self.clrsetreg(
                        clksel_con(41),
                        (1 << 8) | 0xFF,
                        (1 << 8) | (src_clk_div - 1),
                    );
                    OSC_HZ / (src_clk_div as u64)
                } else {
                    let src_clk_div = (self.gpll_hz / rate_hz).min(7) as u32;
                    self.clrsetreg(
                        clksel_con(41),
                        (1 << 8) | 0xFF,
                        ((src_clk_div - 1)),
                    );
                    100 * MHZ / (src_clk_div as u64)
                }
            }
            _ => 0,
        }
    }

    // ========================================================================
    // UART 时钟
    // ========================================================================

    /// 获取 UART 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_uart_get_rate()
    ///
    /// 注意：仅支持 SCLK_UART0-3 (ID: 632-635)
    pub(crate) fn uart_get_rate(&self, id: ClkId) -> u64 {
        let reg = match id {
            SCLK_UART0 => 41,
            SCLK_UART1 => 43,
            SCLK_UART2 => 45,
            SCLK_UART3 => 47,
            _ => return 0,
        };

        let con = self.read(clksel_con(reg + 2));
        let src = con & 0x3;

        let con = self.read(clksel_con(reg));
        let div = ((con >> 9) & 0x1F) as u64;
        let p_src = (con >> 14) & 1;
        let p_rate = if p_src == 0 {
            self.gpll_hz
        } else {
            self.cpll_hz
        };

        match src {
            0 => p_rate / (div + 1), // CLK_UART_SEL_SRC
            1 => {
                // CLK_UART_SEL_FRAC
                let fracdiv = self.read(clksel_con(reg + 1));
                let n = (fracdiv >> 16) & 0xFFFF;
                let m = fracdiv & 0xFFFF;
                (p_rate / (div + 1)) * n as u64 / m as u64
            }
            2 => OSC_HZ, // CLK_UART_SEL_XIN24M
            _ => 0,
        }
    }

    /// 设置 UART 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_uart_set_rate()
    ///
    /// 注意：仅支持 SCLK_UART0-3 (ID: 632-635)
    pub(crate) fn uart_set_rate(&mut self, id: ClkId, rate_hz: u64) -> u64 {
        let reg = match id {
            SCLK_UART0 => 41,
            SCLK_UART1 => 43,
            SCLK_UART2 => 45,
            SCLK_UART3 => 47,
            _ => return 0,
        };

        let (clk_src, uart_src, div) = if self.gpll_hz.is_multiple_of(rate_hz) {
            (0, 0, (self.gpll_hz / rate_hz) as u32) // GPLL, SEL_SRC
        } else if self.cpll_hz.is_multiple_of(rate_hz) {
            (1, 0, (self.cpll_hz / rate_hz) as u32) // CPLL, SEL_SRC
        } else if rate_hz == OSC_HZ {
            (0, 2, 2) // GPLL, SEL_XIN24M
        } else {
            // 小数分频模式 - 简化实现
            (0, 1, 2) // GPLL, SEL_FRAC
        };

        // 配置时钟源和分频
        self.clrsetreg(
            clksel_con(reg),
            (1 << 14) | (0x1F << 9),
            (clk_src << 14) | ((div - 1) << 9),
        );

        // 配置 UART 时钟选择
        self.clrsetreg(clksel_con(reg + 2), 0x3, uart_src);

        match uart_src {
            0 => {
                if clk_src == 0 {
                    self.gpll_hz / div as u64
                } else {
                    self.cpll_hz / div as u64
                }
            }
            2 => OSC_HZ,
            _ => rate_hz,
        }
    }

    // ========================================================================
    // MMC/SDMMC 时钟
    // ========================================================================

    /// 获取 MMC 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_mmc_get_clk()
    ///
    /// 简化实现：返回固定 200MHz
    pub(crate) fn mmc_get_rate(&self, _id: ClkId) -> u64 {
        // MMC 时钟配置复杂，涉及多个分频器和时钟源
        // 完整实现需要读取 CCLK_SRC_EMMC, CCLK_EMMC 等寄存器
        200 * MHZ
    }

    /// 设置 MMC 时钟频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_mmc_set_clk()
    ///
    /// 简化实现：返回 200MHz
    pub(crate) fn mmc_set_rate(&mut self, _id: ClkId, _rate_hz: u64) -> u64 {
        // MMC 时钟设置复杂，完整实现需要：
        // 1. 选择时钟源 (GPLL/CPLL/200MHz/24MHz)
        // 2. 配置分频器 (div, fracdiv)
        // 3. 配置采样时钟
        200 * MHZ
    }

    // ========================================================================
    // 根时钟
    // ========================================================================

    /// 获取根时钟频率
    pub(crate) fn root_clk_get_rate(&self, id: ClkId) -> u64 {
        match id {
            ACLK_BUS_ROOT => {
                let clksel_38 = self.read(clksel_con(38));
                let div = ((clksel_38 & 0x1F) + 1) as u64;
                self.gpll_hz / div
            }
            ACLK_TOP_ROOT | ACLK_LOW_TOP_ROOT => 200 * MHZ,
            PCLK_TOP_ROOT => 100 * MHZ,
            ACLK_CENTER_ROOT | PCLK_CENTER_ROOT | HCLK_CENTER_ROOT | ACLK_CENTER_LOW_ROOT => {
                self.gpll_hz / 2
            }
            _ => OSC_HZ,
        }
    }
}
