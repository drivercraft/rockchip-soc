use crate::clock::pll::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum PllId {
    B0PLL,
    B1PLL,
    LPLL,
    CPLL,
    GPLL,
    NPLL,
    V0PLL,
    AUPLL,
    PPLL,
    _Len,
}

pub const PLL_RATE_TABLE: &[PllRateTable] = &[
    pll_rate(1500000000, 2, 250, 1, 0),
    pll_rate(1200000000, 2, 200, 1, 0),
    pll_rate(1188000000, 2, 198, 1, 0),
    pll_rate(1100000000, 3, 550, 2, 0),
    pll_rate(1008000000, 2, 336, 2, 0),
    pll_rate(1000000000, 3, 500, 2, 0),
    pll_rate(900000000, 2, 300, 2, 0),
    pll_rate(850000000, 3, 425, 2, 0),
    pll_rate(816000000, 2, 272, 2, 0),
    pll_rate(786432000, 2, 262, 2, 9437),
    pll_rate(786000000, 1, 131, 2, 0),
    pll_rate(742500000, 4, 495, 2, 0),
    pll_rate(722534400, 8, 963, 2, 24850),
    pll_rate(600000000, 2, 200, 2, 0),
    pll_rate(594000000, 2, 198, 2, 0),
    pll_rate(200000000, 3, 400, 4, 0),
    pll_rate(100000000, 3, 400, 5, 0),
];

/// RK3588 PLL 时钟配置
///
/// 参考 u-boot-orangepi/drivers/clk/rockchip/clk_rk3588.c
pub const RK3588_PLL_CLOCKS: [PllClock; PllId::_Len as usize] = [
    // B0PLL - BIGCORE0 PLL
    PllClock::new(
        PllId::B0PLL as u32,
        0x50000, // RK3588_B0_PLL_CON(0) = RK3588_BIGCORE0_CRU_BASE + 0*4
        0x50280, // RK3588_B0_PLL_MODE_CON = RK3588_BIGCORE0_CRU_BASE + 0x280
        0,       // mode_shift
        15,      // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // B1PLL - BIGCORE1 PLL
    PllClock::new(
        PllId::B1PLL as u32,
        0x52020, // RK3588_B1_PLL_CON(8) = RK3588_BIGCORE1_CRU_BASE + 8*4
        0x52280, // RK3588_B1_PLL_MODE_CON = RK3588_BIGCORE1_CRU_BASE + 0x280
        0,       // mode_shift
        15,      // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // LPLL - DSU PLL
    PllClock::new(
        PllId::LPLL as u32,
        0x58040, // RK3588_LPLL_CON(16) = RK3588_DSU_CRU_BASE + 16*4
        0x58280, // RK3588_LPLL_MODE_CON = RK3588_DSU_CRU_BASE + 0x280
        0,       // mode_shift
        15,      // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // V0PLL - Video PLL 0
    PllClock::new(
        PllId::V0PLL as u32,
        0x160, // RK3588_PLL_CON(88) = 88*4
        0x280, // RK3588_MODE_CON0
        4,     // mode_shift
        15,    // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // AUPLL - Audio PLL
    PllClock::new(
        PllId::AUPLL as u32,
        0x180, // RK3588_PLL_CON(96) = 96*4
        0x280, // RK3588_MODE_CON0
        6,     // mode_shift
        15,    // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // CPLL - Center PLL
    PllClock::new(
        PllId::CPLL as u32,
        0x1a0, // RK3588_PLL_CON(104) = 104*4
        0x280, // RK3588_MODE_CON0
        8,     // mode_shift
        15,    // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // GPLL - General PLL
    PllClock::new(
        PllId::GPLL as u32,
        0x1c0, // RK3588_PLL_CON(112) = 112*4
        0x280, // RK3588_MODE_CON0
        2,     // mode_shift
        15,    // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // NPLL - New PLL
    PllClock::new(
        PllId::NPLL as u32,
        0x1e0, // RK3588_PLL_CON(120) = 120*4
        0x280, // RK3588_MODE_CON0
        0,     // mode_shift
        15,    // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
    // PPLL - PMU PLL (在 PHP_CRU 基地址)
    PllClock::new(
        PllId::PPLL as u32,
        0x8200, // RK3588_PMU_PLL_CON(128) = RK3588_PHP_CRU_BASE + 128*4
        0x280,  // RK3588_MODE_CON0
        10,     // mode_shift
        15,     // lock_shift
        RockchipPllType::PllRk3588,
        crate::clock::pll::pll_flags::PLL_RK3588,
        PLL_RATE_TABLE,
        0x3, // mode_mask: 2 bits
    ),
];

const fn pll_rate(rate: u64, p: u32, m: u32, s: u32, k: u32) -> PllRateTable {
    PllRateTable {
        rate,
        params: PllRateParams::Rk3588 { p, m, s, k },
    }
}
