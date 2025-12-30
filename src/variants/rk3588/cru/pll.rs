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
    PLL_COUNT,
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

// pub const RK3588_PLLS: [Pll; PllId::PLL_COUNT as usize] =  ;

const fn pll_rate(rate: u64, p: u32, m: u32, s: u32, k: u32) -> PllRateTable {
    PllRateTable {
        rate,
        params: PllRateParams::Rk3588 { p, m, s, k },
    }
}
