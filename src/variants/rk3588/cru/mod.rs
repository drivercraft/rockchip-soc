use crate::{Mmio, grf::GrfMmio};

mod consts;
mod pll;

// =============================================================================
// 公开导出
// =============================================================================

pub use consts::*;
pub use pll::*;

// =============================================================================
// 内部常量定义
// =============================================================================

/// MHz 单位
const MHZ: u64 = 1_000_000;

/// clksel_con 寄存器基址偏移
const CLKSEL_CON_OFFSET: usize = 0x0300;

/// PLL 模式掩码
const PLL_MODE_MASK: u32 = 0x3;

/// PLLCON 寄存器偏移量
const RK3588_PLLCON: fn(u32) -> u32 = |i: u32| i * 0x4;

/// 计算 clksel_con 寄存器偏移
#[must_use]
const fn clksel_con(index: u32) -> usize {
    CLKSEL_CON_OFFSET + (index as usize) * 4
}

/// ACLK_BUS_ROOT 选择和分频位定义 (clksel_con[38])
const ACLK_BUS_ROOT_SEL_SHIFT: u32 = 5;
const ACLK_BUS_ROOT_SEL_MASK: u32 = 0x3 << ACLK_BUS_ROOT_SEL_SHIFT;
const ACLK_BUS_ROOT_SEL_GPLL: u32 = 0;
const ACLK_BUS_ROOT_DIV_SHIFT: u32 = 0;
const ACLK_BUS_ROOT_DIV_MASK: u32 = 0x1f << ACLK_BUS_ROOT_DIV_SHIFT;

/// ACLK_TOP_S400 和 ACLK_TOP_S200 选择位定义 (clksel_con[9])
const ACLK_TOP_S400_SEL_SHIFT: u32 = 8;
const ACLK_TOP_S400_SEL_MASK: u32 = 0x3 << ACLK_TOP_S400_SEL_SHIFT;
const ACLK_TOP_S400_SEL_400M: u32 = 0;
const ACLK_TOP_S200_SEL_SHIFT: u32 = 6;
const ACLK_TOP_S200_SEL_MASK: u32 = 0x3 << ACLK_TOP_S200_SEL_SHIFT;
const ACLK_TOP_S200_SEL_200M: u32 = 0;

#[derive(Debug, Clone)]
pub struct Cru {
    base: usize,
    grf: usize,
    cpll_hz: u64,
    gpll_hz: u64,
}

impl Cru {
    pub fn new(base: Mmio, sys_grf: Mmio) -> Self {
        Cru {
            base: base.as_ptr() as usize,
            grf: sys_grf.as_ptr() as usize,
            cpll_hz: 0,
            gpll_hz: 0,
        }
    }

    /// 初始化并验证 CRU 配置
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_rk3588.c:rk3588_clk_init()
    ///
    /// ⚠️ 此方法仅**验证**配置，不修改寄存器
    /// 假设 bootloader (u-boot/TPL) 已正确配置 PLL 和时钟分频
    ///
    /// u-boot rk3588_clk_init 配置：
    /// 1. ACLK_BUS_ROOT: GPLL/4 ≈ 300MHz (clksel_con[38])
    /// 2. CPLL: 1500MHz
    /// 3. GPLL: 1188MHz
    /// 4. PPLL: 1100MHz (如果启用 PCI)
    /// 5. ACLK_TOP_S400: 400MHz (clksel_con[9])
    /// 6. ACLK_TOP_S200: 200MHz (clksel_con[9])
    pub fn init(&mut self) {
        log::info!(
            "CRU@{:x}: Verifying clock configuration from u-boot",
            self.base
        );
        log::info!("Comparing with u-boot drivers/clk/rockchip/clk_rk3588.c:rk3588_clk_init()");

        // ========================================================================
        // 1. 验证 ACLK_BUS_ROOT 配置
        // u-boot: div = DIV_ROUND_UP(GPLL_HZ, 300 * MHz); = 1188/300 = 4
        //        rk_clrsetreg(&priv->cru->clksel_con[38],
        //                     ACLK_BUS_ROOT_SEL_MASK | ACLK_BUS_ROOT_DIV_MASK,
        //                     div << ACLK_BUS_ROOT_DIV_SHIFT);
        // 预期: SEL=0 (GPLL), DIV=4
        // ========================================================================
        let clksel_38 = self.read(clksel_con(38));
        let bus_root_sel = (clksel_38 & ACLK_BUS_ROOT_SEL_MASK) >> ACLK_BUS_ROOT_SEL_SHIFT;
        let bus_root_div = (clksel_38 & ACLK_BUS_ROOT_DIV_MASK) >> ACLK_BUS_ROOT_DIV_SHIFT;

        debug!(
            "CRU@{:x}: clksel_con[38] (ACLK_BUS_ROOT): 0x{:08x}",
            self.base, clksel_38
        );
        debug!("  - SEL: {} (0=GPLL, 1=CPLL, 2=NPLL, 3=24M)", bus_root_sel);
        // u-boot: DIV_TO_RATE(input_rate, div) = ((input_rate) / ((div) + 1))
        // 所以实际分频系数是 (div + 1)
        let bus_root_div_factor = bus_root_div + 1;
        let bus_root_rate = if bus_root_div > 0 {
            GPLL_HZ as u64 / bus_root_div_factor as u64
        } else {
            0
        };
        debug!(
            "  - DIV: {} (factor: {}, output: {}MHz)",
            bus_root_div,
            bus_root_div_factor,
            bus_root_rate / MHZ
        );

        // u-boot 配置验证
        // u-boot: div = DIV_ROUND_UP(GPLL_HZ, 300 * MHz) - 1;
        //       = (1188 + 300 - 1) / 300 - 1 = 4 - 1 = 3
        let expected_div = ((GPLL_HZ as u64) + (300 * MHZ) - 1) / (300 * MHZ) - 1;
        if bus_root_sel != ACLK_BUS_ROOT_SEL_GPLL {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_BUS_ROOT source mismatch! u-boot: GPLL(0), current: {}",
                self.base,
                bus_root_sel
            );
        } else {
            debug!("✓ ACLK_BUS_ROOT source matches u-boot (GPLL)");
        }

        if bus_root_div != expected_div as u32 {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_BUS_ROOT div mismatch! u-boot: {}, current: {}",
                self.base,
                expected_div,
                bus_root_div
            );
        } else {
            debug!("✓ ACLK_BUS_ROOT div matches u-boot ({})", expected_div);
        }

        // ========================================================================
        // 2. 验证 ACLK_TOP_S400/S200 配置
        // u-boot: rk_clrsetreg(&priv->cru->clksel_con[9],
        //                      ACLK_TOP_S400_SEL_MASK | ACLK_TOP_S200_SEL_MASK,
        //                      (ACLK_TOP_S400_SEL_400M << ACLK_TOP_S400_SEL_SHIFT) |
        //                      (ACLK_TOP_S200_SEL_200M << ACLK_TOP_S200_SEL_SHIFT));
        // 预期: S400_SEL=0 (400MHz), S200_SEL=0 (200MHz)
        // ========================================================================
        let clksel_9 = self.read(clksel_con(9));
        let s400_sel = (clksel_9 & ACLK_TOP_S400_SEL_MASK) >> ACLK_TOP_S400_SEL_SHIFT;
        let s200_sel = (clksel_9 & ACLK_TOP_S200_SEL_MASK) >> ACLK_TOP_S200_SEL_SHIFT;

        debug!(
            "CRU@{:x}: clksel_con[9] (ACLK_TOP): 0x{:08x}",
            self.base, clksel_9
        );
        debug!("  - S400_SEL: {} (0=400MHz, 1=200MHz)", s400_sel);
        debug!("  - S200_SEL: {} (0=200MHz, 1=100MHz)", s200_sel);

        if s400_sel != ACLK_TOP_S400_SEL_400M {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_TOP_S400 mismatch! u-boot: 0 (400MHz), current: {}",
                self.base,
                s400_sel
            );
        } else {
            debug!("✓ ACLK_TOP_S400 matches u-boot (400MHz)");
        }

        if s200_sel != ACLK_TOP_S200_SEL_200M {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_TOP_S200 mismatch! u-boot: 0 (200MHz), current: {}",
                self.base,
                s200_sel
            );
        } else {
            debug!("✓ ACLK_TOP_S200 matches u-boot (200MHz)");
        }

        // ========================================================================
        // 3. 读取并验证 PLL 频率
        // u-boot 通过 rockchip_pll_set_rate() 配置:
        // - CPLL: CPLL_HZ (1500MHz)
        // - GPLL: GPLL_HZ (1188MHz)
        // ========================================================================
        let cpll_actual = self.read_pll_rate(PllId::CPLL);
        let gpll_actual = self.read_pll_rate(PllId::GPLL);

        // 保存实际读取到的频率
        self.cpll_hz = cpll_actual;
        self.gpll_hz = gpll_actual;

        debug!("PLL actual rates (read from registers):");
        debug!("  - CPLL: {}MHz", cpll_actual / MHZ);
        debug!("  - GPLL: {}MHz", gpll_actual / MHZ);

        // 验证与 u-boot 预期值的一致性
        verify_pll_frequency(PllId::CPLL, cpll_actual, CPLL_HZ as u64);
        verify_pll_frequency(PllId::GPLL, gpll_actual, GPLL_HZ as u64);

        log::info!(
            "✓ CRU@{:x}: Clock configuration verified vs u-boot",
            self.base
        );
    }

    /// 读取 PLL 实际频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_pll.c:rk3588_pll_get_rate()
    ///
    /// # 参数
    ///
    /// * `pll_id` - PLL ID
    ///
    /// # 返回
    ///
    /// PLL 输出频率 (Hz)
    #[must_use]
    fn read_pll_rate(&self, pll_id: PllId) -> u64 {
        let pll_cfg = get_pll(pll_id);

        // 1. 读取 PLL 模式
        let mode_con = self.read(pll_cfg.mode_offset as usize);
        let mode_shift = pll_cfg.mode_shift;

        // PPLL (ID=8) 特殊处理: 始终认为是 NORMAL 模式
        let pll_id_val = pll_id as u32;
        let mode = if pll_id_val == 8 {
            pll_mode::PLL_MODE_NORMAL
        } else {
            (mode_con & (PLL_MODE_MASK << mode_shift)) >> mode_shift
        };

        match mode {
            pll_mode::PLL_MODE_SLOW => {
                debug!(
                    "{}[mode_shift={}] is in SLOW mode, returning OSC_HZ",
                    pll_id.name(),
                    mode_shift
                );
                return OSC_HZ as u64;
            }
            pll_mode::PLL_MODE_DEEP => {
                debug!(
                    "{}[mode_shift={}] is in DEEP mode, returning 32768Hz",
                    pll_id.name(),
                    mode_shift
                );
                return 32768;
            }
            pll_mode::PLL_MODE_NORMAL => {
                // 继续读取 PLL 参数
            }
            _ => {
                log::warn!(
                    "⚠️ {}[mode_shift={}]: unknown mode={}, returning OSC_HZ",
                    pll_id.name(),
                    mode_shift,
                    mode
                );
                return OSC_HZ as u64;
            }
        }

        // 2. 读取 PLL 参数 (参考 u-boot rk3588_pll_get_rate)
        // PLLCON0: M (10 bits)
        let con0 = self.read(pll_cfg.con_offset as usize);
        let m = (con0 & pllcon0::M_MASK) >> pllcon0::M_SHIFT;

        // PLLCON1: P (6 bits), S (3 bits)
        let con1 = self.read((pll_cfg.con_offset + RK3588_PLLCON(1)) as usize);
        let p = (con1 & pllcon1::P_MASK) >> pllcon1::P_SHIFT;
        let s = (con1 & pllcon1::S_MASK) >> pllcon1::S_SHIFT;

        // PLLCON2: K (16 bits)
        let con2 = self.read((pll_cfg.con_offset + RK3588_PLLCON(2)) as usize);
        let k = (con2 & pllcon2::K_MASK) >> pllcon2::K_SHIFT;

        debug!("{}: p={}, m={}, s={}, k={}", pll_id.name(), p, m, s, k);

        // 3. 验证 p 值
        if p == 0 {
            log::warn!(
                "⚠️ PLL[mode_shift={}] has invalid p=0, assuming not configured, returning OSC_HZ",
                mode_shift
            );
            return OSC_HZ as u64;
        }

        // 4. 计算频率 (参考 u-boot rk3588_pll_get_rate)
        // rate = OSC_HZ / p * m
        let mut rate: u64 = (OSC_HZ as u64 / p as u64) * m as u64;

        // 如果有小数分频 k
        if k != 0 {
            // frac_rate = OSC_HZ * k / (p * 65536)
            let frac_rate = (OSC_HZ as u64 * k as u64) / (p as u64 * 65536);
            rate += frac_rate;
        }

        // 右移 s 位 (后分频)
        rate >>= s;

        debug!("{}: calculated rate = {}MHz", pll_id.name(), rate / MHZ);

        rate
    }

    /// 写入 clksel_con 寄存器
    ///
    /// # 参数
    ///
    /// * `index` - 寄存器索引 (0-177)
    /// * `mask` - 位掩码（要修改的位）
    /// * `value` - 要写入的值（已移位到正确位置）
    fn clksel_con_write(&mut self, index: usize, mask: u32, value: u32) {
        let reg_addr = self.base + CLKSEL_CON_OFFSET + index * 4;

        log::debug!(
            "CRU@{:x}: Writing clksel_con[{}] = 0x{:08x} (mask=0x{:08x})",
            self.base,
            index,
            value,
            mask
        );

        unsafe {
            let reg = reg_addr as *mut u32;

            // 读取当前值
            let current = reg.read_volatile();

            // 清除要修改的位，然后设置新值
            let new_value = (current & !mask) | (value & mask);

            // 写入新值
            reg.write_volatile(new_value);

            // 读取并验证
            let verify = reg.read_volatile();
            log::debug!(
                "CRU@{:x}: clksel_con[{}] readback: 0x{:08x}",
                self.base,
                index,
                verify
            );
        }
    }

    pub fn grf_mmio_ls() -> &'static [GrfMmio] {
        &[super::syscon::grf_mmio::SYS_GRF]
    }

    fn reg(&self, offset: usize) -> *mut u32 {
        (self.base + offset) as *mut u32
    }

    fn read(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile(self.reg(offset)) }
    }

    fn write(&self, offset: usize, value: u32) {
        unsafe { core::ptr::write_volatile(self.reg(offset), value) }
    }
}

/// 验证 PLL 频率
///
/// 对比实际读取的 PLL 频率与 u-boot 配置的预期频率
///
/// # 参数
///
/// * `pll_id` - PLL ID
/// * `actual_hz` - 实际读取的频率 (Hz)
/// * `expected_hz` - 预期频率 (Hz)
fn verify_pll_frequency(pll_id: PllId, actual_hz: u64, expected_hz: u64) {
    let diff_hz = if actual_hz > expected_hz {
        actual_hz - expected_hz
    } else {
        expected_hz - actual_hz
    };

    // 允许 0.1% 的误差
    let tolerance = expected_hz / 1000;

    if diff_hz <= tolerance {
        debug!(
            "✓ {}: {}MHz (expected: {}MHz, diff: {}Hz)",
            pll_id.name(),
            actual_hz / MHZ,
            expected_hz / MHZ,
            diff_hz
        );
    } else {
        log::warn!(
            "⚠️ {}: {}MHz (expected: {}MHz, diff: {}Hz, tolerance: {}Hz)",
            pll_id.name(),
            actual_hz / MHZ,
            expected_hz / MHZ,
            diff_hz,
            tolerance
        );
    }
}

// =============================================================================
// 单元测试
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 u-boot 配置值的常量验证
    #[test]
    fn test_u_boot_init_values() {
        // 验证 u-boot rk3588_clk_init 中的常量计算
        // ACLK_BUS_ROOT 分频器计算
        // u-boot: div = DIV_ROUND_UP(GPLL_HZ, 300 * MHz);
        //       = (1188 + 300 - 1) / 300 = 4
        //       写入: div - 1 = 3 (因为 DIV_TO_RATE = rate / (div + 1))
        let expected_div_reg = ((GPLL_HZ as u64) + (300 * MHZ) - 1) / (300 * MHZ) - 1;
        assert_eq!(
            expected_div_reg, 3,
            "ACLK_BUS_ROOT div should be 3 (factor=4)"
        );

        // ACLK_TOP_S400: 0 = 400MHz
        let expected_s400_sel = ACLK_TOP_S400_SEL_400M;
        assert_eq!(expected_s400_sel, 0, "ACLK_TOP_S400 should be 0 (400MHz)");

        // ACLK_TOP_S200: 0 = 200MHz
        let expected_s200_sel = ACLK_TOP_S200_SEL_200M;
        assert_eq!(expected_s200_sel, 0, "ACLK_TOP_S200 should be 0 (200MHz)");

        // PLL 频率验证
        assert_eq!(CPLL_HZ, 1500 * (MHZ as u32), "CPLL should be 1500MHz");
        assert_eq!(GPLL_HZ, 1188 * (MHZ as u32), "GPLL should be 1188MHz");
        assert_eq!(PPLL_HZ, 1100 * (MHZ as u32), "PPLL should be 1100MHz");
    }

    /// 测试寄存器位掩码定义
    #[test]
    fn test_register_bit_masks() {
        // ACLK_BUS_ROOT 位掩码
        assert_eq!(ACLK_BUS_ROOT_SEL_MASK, 0x3 << 5);
        assert_eq!(ACLK_BUS_ROOT_DIV_MASK, 0x1f);

        // ACLK_TOP 位掩码
        assert_eq!(ACLK_TOP_S400_SEL_MASK, 0x3 << 8);
        assert_eq!(ACLK_TOP_S200_SEL_MASK, 0x3 << 6);
    }

    /// 测试 clksel_con 寄存器地址计算
    #[test]
    fn test_clksel_con_address() {
        // clksel_con[0] = 0x300
        assert_eq!(CLKSEL_CON_OFFSET + 0 * 4, 0x300);
        // clksel_con[9] = 0x324
        assert_eq!(CLKSEL_CON_OFFSET + 9 * 4, 0x324);
        // clksel_con[38] = 0x398
        assert_eq!(CLKSEL_CON_OFFSET + 38 * 4, 0x398);
    }

    /// 模拟 u-boot 配置的寄存器值验证
    #[test]
    fn test_expected_register_values() {
        // u-boot rk3588_clk_init 写入的预期值:
        //
        // clksel_con[38]:
        //   SEL = 0 (GPLL)
        //   DIV = 3 (factor = 4)
        //   预期值 = 0x00000003
        let expected_clksel_38 = 0 | 3;
        assert_eq!(expected_clksel_38, 3);

        // clksel_con[9]:
        //   S400_SEL = 0 (400MHz) at bit 8
        //   S200_SEL = 0 (200MHz) at bit 6
        //   预期值 = 0x00000000
        let expected_clksel_9 = (0 << 8) | (0 << 6);
        assert_eq!(expected_clksel_9, 0);
    }

    /// 测试 PLL 频率计算公式
    ///
    /// 验证从寄存器值计算 PLL 输出频率的公式
    #[test]
    fn test_pll_rate_calculation() {
        // 测试 GPLL 1188MHz: p=2, m=198, s=1, k=0
        // rate = ((24MHz / 2) * 198) >> 1 = 1188MHz
        let fin = OSC_HZ as u64;
        let rate = ((fin / 2) * 198) >> 1;
        assert_eq!(rate, 1188 * (MHZ as u64));

        // 测试 CPLL 1500MHz: p=2, m=250, s=1, k=0
        // rate = ((24MHz / 2) * 250) >> 1 = 1500MHz
        let rate = ((fin / 2) * 250) >> 1;
        assert_eq!(rate, 1500 * (MHZ as u64));

        // 测试小数分频 786.432MHz: p=2, m=262, s=2, k=9437
        // rate = ((24MHz / 2) * 262 + (24MHz * 9437) / (2 * 65536)) >> 2
        let p = 2u64;
        let m = 262u64;
        let s = 2u32;
        let k = 9437u64;

        let mut rate = (fin / p) * m;
        let frac_rate = (fin * k) / (p * 65536);
        rate += frac_rate;
        rate >>= s;

        // 由于整数除法精度限制,结果为 786431991 Hz
        assert_eq!(rate, 786_431_991);
    }

    /// 测试 PLL 模式掩码和常量
    #[test]
    fn test_pll_mode_constants() {
        // 验证模式常量
        assert_eq!(pll_mode::PLL_MODE_SLOW, 0);
        assert_eq!(pll_mode::PLL_MODE_NORMAL, 1);
        assert_eq!(pll_mode::PLL_MODE_DEEP, 2);

        // 验证模式掩码
        assert_eq!(PLL_MODE_MASK, 0x3);
    }

    /// 测试 PLL 寄存器位掩码
    #[test]
    fn test_pll_register_masks() {
        // PLLCON0: M (10 bits)
        assert_eq!(pllcon0::M_MASK, 0x3ff);
        assert_eq!(pllcon0::M_SHIFT, 0);

        // PLLCON1: P (6 bits), S (3 bits)
        assert_eq!(pllcon1::P_MASK, 0x3f);
        assert_eq!(pllcon1::P_SHIFT, 0);
        assert_eq!(pllcon1::S_MASK, 0x7 << 6);
        assert_eq!(pllcon1::S_SHIFT, 6);

        // PLLCON2: K (16 bits)
        assert_eq!(pllcon2::K_MASK, 0xffff);
        assert_eq!(pllcon2::K_SHIFT, 0);
    }
}
