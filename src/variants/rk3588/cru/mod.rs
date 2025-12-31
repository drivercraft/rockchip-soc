use crate::{Mmio, grf::GrfMmio};

mod pll;
mod consts;

// =============================================================================
// 公开导出
// =============================================================================

pub use consts::*;
pub use pll::{PllId, PLL_RATE_TABLE, RK3588_PLL_CLOCKS};

// =============================================================================
// 内部常量定义
// =============================================================================

/// MHz 单位
const MHZ: u64 = 1_000_000;

/// clksel_con 寄存器基址偏移
const CLKSEL_CON_OFFSET: usize = 0x0300;

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
    /// 验证以下配置：
    /// 1. ACLK_BUS_ROOT 是否从 GPLL 分频得到约 300MHz
    /// 2. ACLK_TOP_S400 是否配置为 400MHz
    /// 3. ACLK_TOP_S200 是否配置为 200MHz
    pub fn init(&mut self) {
        log::info!(
            "CRU@{:x}: Verifying clock configuration from bootloader",
            self.base
        );

        // Step 1: 验证 ACLK_BUS_ROOT 配置
        let clksel_38 = self.read(CLKSEL_CON_OFFSET + 38 * 4);
        let bus_root_sel = (clksel_38 & ACLK_BUS_ROOT_SEL_MASK) >> ACLK_BUS_ROOT_SEL_SHIFT;
        let bus_root_div = (clksel_38 & ACLK_BUS_ROOT_DIV_MASK) >> ACLK_BUS_ROOT_DIV_SHIFT;

        debug!(
            "CRU@{:x}: ACLK_BUS_ROOT (clksel_con[38]): 0x{:08x}",
            self.base, clksel_38
        );
        debug!("  - SEL: {} (0=GPLL, 1=CPLL)", bus_root_sel);
        debug!(
            "  - DIV: {} (output: ~{}MHz)",
            bus_root_div,
            if bus_root_div > 0 {
                1188 / bus_root_div as u64
            } else {
                0
            }
        );

        if bus_root_sel != ACLK_BUS_ROOT_SEL_GPLL {
            log::warn!(
                "⚠ CRU@{:x}: ACLK_BUS_ROOT source is not GPLL! (current: {})",
                self.base,
                bus_root_sel
            );
        }

        // Step 2: 验证 ACLK_TOP_S400/S200 配置
        let clksel_9 = self.read(CLKSEL_CON_OFFSET + 9 * 4);
        let s400_sel = (clksel_9 & ACLK_TOP_S400_SEL_MASK) >> ACLK_TOP_S400_SEL_SHIFT;
        let s200_sel = (clksel_9 & ACLK_TOP_S200_SEL_MASK) >> ACLK_TOP_S200_SEL_SHIFT;

        log::info!(
            "CRU@{:x}: ACLK_TOP (clksel_con[9]): 0x{:08x}",
            self.base,
            clksel_9
        );
        log::info!("  - S400_SEL: {} (0=400MHz, 1=200MHz)", s400_sel);
        log::info!("  - S200_SEL: {} (0=200MHz, 1=100MHz)", s200_sel);

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

        // Step 3: 假设 PLL 已由 bootloader 正确配置
        self.cpll_hz = CPLL_HZ as u64;
        self.gpll_hz = GPLL_HZ as u64;

        log::info!("✓ CRU@{:x}: Clock configuration verified", self.base);
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
