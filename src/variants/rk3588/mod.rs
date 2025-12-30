use crate::{Mmio, grf::GrfMmio};

mod syscon;

// =============================================================================
// 常量定义
// =============================================================================

/// MHz 单位
const MHZ: u64 = 1_000_000;

/// OSC 时钟频率 (24MHz) - PLL 参考时钟
const OSC_HZ: u64 = 24 * MHZ;

/// GPLL 目标频率 (1188 MHz)
const GPLL_HZ: u64 = 1188 * MHZ;

/// CPLL 目标频率 (600 MHz)
const CPLL_HZ: u64 = 600 * MHZ;

/// clksel_con 寄存器基址偏移
const CLKSEL_CON_OFFSET: usize = 0x0300;

/// PLL con 寄存器基址偏移
const PLL_CON_OFFSET: usize = 0x0;

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

// =============================================================================
// PLL 寄存器位定义 (参考 u-boot clk_pll.c)
// =============================================================================

/// RK3588 PLL_CON0 位定义
const PLLCON0_M_SHIFT: u32 = 0;
const PLLCON0_M_MASK: u32 = 0xfff << PLLCON0_M_SHIFT;

/// RK3588 PLL_CON1 位定义
const PLLCON1_P_SHIFT: u32 = 0;
const PLLCON1_P_MASK: u32 = 0x3f << PLLCON1_P_SHIFT;
const PLLCON1_S_SHIFT: u32 = 6;
const PLLCON1_S_MASK: u32 = 0x7 << PLLCON1_S_SHIFT;

/// RK3588 PLL_CON2 位定义
const PLLCON2_K_SHIFT: u32 = 0;
const PLLCON2_K_MASK: u32 = 0xffff << PLLCON2_K_SHIFT;

/// Mode register offset (from PLL base)
const MODE_CON_OFFSET: usize = 0x280;

/// PLL mode values
const PLL_MODE_SLOW: u32 = 0;
const PLL_MODE_NORMAL: u32 = 1;
const PLL_MODE_DEEP: u32 = 2;

/// Mode mask and shift
const PLL_MODE_MASK: u32 = 0x3;

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
    /// 验证以下配置：
    /// 1. 读取并显示 CPLL 和 GPLL 实际频率
    /// 2. ACLK_BUS_ROOT 是否从 GPLL 分频得到约 300MHz
    /// 3. ACLK_TOP_S400 是否配置为 400MHz
    /// 4. ACLK_TOP_S200 是否配置为 200MHz
    pub fn init(&mut self) {
        log::info!("CRU@{:x}: Verifying clock configuration from bootloader", self.base);

        // Step 1: 读取 PLL 实际频率
        // CPLL: mode_shift=8, con_offset=104
        self.cpll_hz = self.get_pll_rate(8, 104);
        // GPLL: mode_shift=2, con_offset=112
        self.gpll_hz = self.get_pll_rate(2, 112);

        log::info!(
            "CRU@{:x}: PLL frequencies: CPLL={}MHz, GPLL={}MHz",
            self.base,
            self.cpll_hz / MHZ,
            self.gpll_hz / MHZ
        );

        // Step 2: 验证 ACLK_BUS_ROOT 配置
        let clksel_38 = self.read(CLKSEL_CON_OFFSET + 38 * 4);
        let bus_root_sel = (clksel_38 & ACLK_BUS_ROOT_SEL_MASK) >> ACLK_BUS_ROOT_SEL_SHIFT;
        let bus_root_div = (clksel_38 & ACLK_BUS_ROOT_DIV_MASK) >> ACLK_BUS_ROOT_DIV_SHIFT;

        log::info!(
            "CRU@{:x}: ACLK_BUS_ROOT (clksel_con[38]): 0x{:08x}",
            self.base,
            clksel_38
        );
        log::info!(
            "  - SEL: {} (0=GPLL, 1=CPLL)",
            bus_root_sel
        );
        log::info!(
            "  - DIV: {} (output: ~{}MHz)",
            bus_root_div,
            if bus_root_div > 0 {
                let src = if bus_root_sel == 0 { self.gpll_hz } else { self.cpll_hz };
                src / (bus_root_div as u64 * MHZ)
            } else { 0 }
        );

        if bus_root_sel != ACLK_BUS_ROOT_SEL_GPLL {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_BUS_ROOT source is not GPLL! (current: {})",
                self.base,
                bus_root_sel
            );
        }

        // Step 3: 验证 ACLK_TOP_S400/S200 配置
        let clksel_9 = self.read(CLKSEL_CON_OFFSET + 9 * 4);
        let s400_sel = (clksel_9 & ACLK_TOP_S400_SEL_MASK) >> ACLK_TOP_S400_SEL_SHIFT;
        let s200_sel = (clksel_9 & ACLK_TOP_S200_SEL_MASK) >> ACLK_TOP_S200_SEL_SHIFT;

        log::info!(
            "CRU@{:x}: ACLK_TOP (clksel_con[9]): 0x{:08x}",
            self.base,
            clksel_9
        );
        log::info!(
            "  - S400_SEL: {} (0=400MHz, 1=200MHz)",
            s400_sel
        );
        log::info!(
            "  - S200_SEL: {} (0=200MHz, 1=100MHz)",
            s200_sel
        );

        if s400_sel != ACLK_TOP_S400_SEL_400M {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_TOP_S400 not set to 400MHz! (current: {})",
                self.base,
                s400_sel
            );
        }

        if s200_sel != ACLK_TOP_S200_SEL_200M {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_TOP_S200 not set to 200MHz! (current: {})",
                self.base,
                s200_sel
            );
        }

        log::info!("✓ CRU@{:x}: Clock configuration verified", self.base);
    }

    /// 读取 PLL 频率
    ///
    /// 参考 u-boot: drivers/clk/rockchip/clk_pll.c:rk3588_pll_get_rate()
    ///
    /// # 参数
    ///
    /// * `mode_shift` - PLL 模式位的移位值 (例如: CPLL=8, GPLL=2)
    /// * `con_offset` - PLL_CON 寄存器偏移量 (例如: CPLL=104, GPLL=112)
    ///
    /// # 返回
    ///
    /// PLL 输出频率 (Hz)
    fn get_pll_rate(&self, mode_shift: u32, con_offset: usize) -> u64 {
        // 检查 PLL 模式
        let mode_con = self.read(MODE_CON_OFFSET);
        let mode = (mode_con & (PLL_MODE_MASK << mode_shift)) >> mode_shift;

        match mode {
            PLL_MODE_SLOW => {
                log::debug!("CRU@{:x}: PLL[mode_shift={}] in SLOW mode, returning OSC_HZ", self.base, mode_shift);
                return OSC_HZ;
            }
            PLL_MODE_DEEP => {
                log::warn!("CRU@{:x}: PLL[mode_shift={}] in DEEP mode, returning 32768Hz", self.base, mode_shift);
                return 32768;
            }
            PLL_MODE_NORMAL => {
                // 正常模式，读取 PLL 参数
            }
            _ => {
                log::warn!("CRU@{:x}: PLL[mode_shift={}] unknown mode {}, returning OSC_HZ", self.base, mode_shift, mode);
                return OSC_HZ;
            }
        }

        // 读取 PLL_CON0/1/2
        let con0 = self.read(PLL_CON_OFFSET + con_offset);
        let m = ((con0 & PLLCON0_M_MASK) >> PLLCON0_M_SHIFT) as u64;

        let con1 = self.read(PLL_CON_OFFSET + con_offset + 4);
        let p = ((con1 & PLLCON1_P_MASK) >> PLLCON1_P_SHIFT) as u64;
        let s = ((con1 & PLLCON1_S_MASK) >> PLLCON1_S_SHIFT) as u64;

        let con2 = self.read(PLL_CON_OFFSET + con_offset + 8);
        let k = ((con2 & PLLCON2_K_MASK) >> PLLCON2_K_SHIFT) as u64;

        // 检查参数合法性
        if p == 0 {
            log::warn!(
                "CRU@{:x}: PLL[mode_shift={}] has invalid p=0, assuming not configured, returning OSC_HZ",
                self.base,
                mode_shift
            );
            return OSC_HZ;
        }

        // 计算频率: FOUT = (OSC_HZ / p) * m >> s (+ frac if k != 0)
        let mut rate = OSC_HZ / p;
        rate *= m;

        if k != 0 {
            // 分数模式
            // frac_rate = (OSC_HZ * k) / (p * 65536)
            let frac_rate = (OSC_HZ * k) / (p * 65536);
            rate += frac_rate;
        }

        rate >>= s;

        log::debug!(
            "CRU@{:x}: PLL[mode_shift={},con_offset={:x}]: mode={}, m={}, p={}, s={}, k={} => {}MHz",
            self.base,
            mode_shift,
            con_offset,
            mode,
            m,
            p,
            s,
            k,
            rate / MHZ
        );

        rate
    }

    pub fn grf_mmio_ls() -> &'static [GrfMmio] {
        &[syscon::grf_mmio::SYS_GRF]
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
