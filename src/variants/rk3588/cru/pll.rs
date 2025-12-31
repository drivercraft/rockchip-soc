//! RK3588 PLL 时钟配置
//!
//! 参考 u-boot-orangepi/drivers/clk/rockchip/clk_rk3588.c

use crate::{clock::pll::*, rk3588::cru::consts::*};

/// RK3588 PLL 时钟 ID
///
/// 对应 u-boot 中的 enum rk3588_pll_id (cru_rk3588.h:22)
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum PllId {
    /// BIGCORE0 PLL - 大核0 PLL
    B0PLL,
    /// BIGCORE1 PLL - 大核1 PLL
    B1PLL,
    /// DSU PLL - 小核共享单元 PLL
    LPLL,
    /// 中心/通用 PLL
    CPLL,
    /// 通用 PLL
    GPLL,
    /// 网络/视频 PLL
    NPLL,
    /// 视频 PLL
    V0PLL,
    /// 音频 PLL
    AUPLL,
    /// PMU PLL
    PPLL,
    /// PLL 总数
    _Len,
}

impl PllId {
    /// 获取 PLL 名称
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::B0PLL => "B0PLL",
            Self::B1PLL => "B1PLL",
            Self::LPLL => "LPLL",
            Self::CPLL => "CPLL",
            Self::GPLL => "GPLL",
            Self::NPLL => "NPLL",
            Self::V0PLL => "V0PLL",
            Self::AUPLL => "AUPLL",
            Self::PPLL => "PPLL",
            Self::_Len => "INVALID",
        }
    }

    /// 获取 PLL 默认频率 (Hz)
    ///
    /// 参考 cru_rk3588.h:15-19
    #[must_use]
    pub const fn default_rate(&self) -> Option<u64> {
        match self {
            Self::B0PLL | Self::B1PLL => Some(LPLL_HZ as u64),
            Self::LPLL => Some(LPLL_HZ as u64),
            Self::GPLL => Some(GPLL_HZ as u64),
            Self::CPLL => Some(CPLL_HZ as u64),
            Self::NPLL => Some(NPLL_HZ as u64),
            Self::PPLL => Some(PPLL_HZ as u64),
            _ => None,
        }
    }
}

/// RK3588 PLL 预设频率表
///
/// 参考 clk_rk3588.c:24
///
/// 支持的频率范围: 100MHz - 1.5GHz
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

macro_rules! pll {
    ($id:ident, $con:expr, $mode:expr, $mshift:expr, $lshift:expr, $pflags:expr) => {
        PllClock {
            id: PllId::$id as u32 + 1,
            con_offset: $con,
            mode_offset: $mode,
            mode_shift: $mshift,
            lock_shift: $lshift,
            pll_type: RockchipPllType::Rk3588,
            pll_flags: $pflags,
            rate_table: PLL_RATE_TABLE,
            mode_mask: 0,
        }
    };
}

/// RK3588 PLL 时钟配置
///
/// 参考 u-boot-orangepi/drivers/clk/rockchip/clk_rk3588.c:46
///
/// RK3588 共有 9 个 PLL:
/// - B0PLL/B1PLL: 大核 PLL (BIGCORE0/1)
/// - LPLL: 小核 PLL (DSU)
/// - V0PLL: 视频 PLL
/// - AUPLL: 音频 PLL
/// - CPLL: 中心/通用 PLL
/// - GPLL: 通用 PLL
/// - NPLL: 网络/视频 PLL
/// - PPLL: PMU PLL
pub const RK3588_PLL_CLOCKS: [PllClock; PllId::_Len as usize] = [
    // B0PLL - BIGCORE0 PLL (偏移 0x50000)
    pll!(B0PLL, b0_pll_con(0), RK3588_B0_PLL_MODE_CON, 0, 15, 0),
    // B1PLL - BIGCORE1 PLL (偏移 0x52000)
    pll!(B1PLL, b1_pll_con(8), RK3588_B1_PLL_MODE_CON, 0, 15, 0),
    // LPLL - DSU PLL (偏移 0x58000)
    pll!(LPLL, lpll_con(16), RK3588_LPLL_MODE_CON, 0, 15, 0),
    // V0PLL - 视频 PLL (偏移 0x160)
    pll!(V0PLL, pll_con(88), RK3588_MODE_CON0, 4, 15, 0),
    // AUPLL - 音频 PLL (偏移 0x180)
    pll!(AUPLL, pll_con(96), RK3588_MODE_CON0, 6, 15, 0),
    // CPLL - 中心/通用 PLL (偏移 0x1a0)
    pll!(CPLL, pll_con(104), RK3588_MODE_CON0, 8, 15, 0),
    // GPLL - 通用 PLL (偏移 0x1c0)
    pll!(GPLL, pll_con(112), RK3588_MODE_CON0, 2, 15, 0),
    // NPLL - 网络/视频 PLL (偏移 0x1e0)
    pll!(NPLL, pll_con(120), RK3588_MODE_CON0, 0, 15, 0),
    // PPLL - PMU PLL (偏移 0x8000)
    pll!(PPLL, pmu_pll_con(128), RK3588_MODE_CON0, 10, 15, 0),
];

/// 创建 RK3588 PLL 速率表项
///
/// # 参数
///
/// * `rate` - 目标输出频率 (Hz)
/// * `p` - P 分频系数 (Pre-divider)
/// * `m` - M 分频系数 (Main Divider)
/// * `s` - S 分频系数 (Post-divider)
/// * `k` - K 小数分频系数
///
/// # 示例
///
/// ```rust
/// let rate = pll_rate(1188_000_000, 2, 198, 1, 0);
/// ```
const fn pll_rate(rate: u64, p: u32, m: u32, s: u32, k: u32) -> PllRateTable {
    PllRateTable {
        rate,
        params: PllRateParams::Rk3588 { p, m, s, k },
    }
}

/// 通过 ID 获取 PLL 配置
///
/// # 参数
///
/// * `id` - PLL ID
///
/// # 返回
///
/// 返回对应 PLL 的配置引用
#[must_use]
pub const fn get_pll(id: PllId) -> &'static PllClock {
    &RK3588_PLL_CLOCKS[id as usize]
}

/// 计算 RK3588 PLL 输出频率
///
/// # 公式
///
/// ```text
/// FOUT = (FIN * M) / (P * (2^S))
/// ```
///
/// 当启用小数分频时 (K != 0):
/// ```text
/// FOUT = (FIN * (M + K/65536)) / (P * (2^S))
/// ```
///
/// # 参数
///
/// * `fin` - 输入频率 (Hz), 通常为 24MHz
/// * `p` - P 分频系数
/// * `m` - M 分频系数
/// * `s` - S 分频系数
/// * `k` - K 小数分频系数
///
/// # 返回
///
/// 计算得到的输出频率 (Hz)
#[must_use]
pub const fn calc_pll_rate(fin: u64, p: u32, m: u32, s: u32, k: u32) -> u64 {
    let p = p as u64;
    let m = m as u64;
    let s = 1u64 << s; // 2^S

    if k != 0 {
        // 小数分频模式
        let k_frac = (k as u64) * fin;
        let k_div = k_frac / 65536;
        (fin * m + k_div) / (p * s)
    } else {
        // 整数分频模式
        (fin * m) / (p * s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pll_rate_table_count() {
        // 验证频率表项数量正确 (17 项)
        assert_eq!(PLL_RATE_TABLE.len(), 17);
    }

    #[test]
    fn test_pll_rate_calculation() {
        // 测试整数分频
        // fin=24MHz, p=2, m=198, s=1, k=0 => 24*198/(2*2) = 1188MHz
        let rate = calc_pll_rate(24_000_000, 2, 198, 1, 0);
        assert_eq!(rate, 1_188_000_000);

        // 测试小数分频
        // 参考 clk_rk3588.c:35 - 786.432MHz
        let rate = calc_pll_rate(24_000_000, 2, 262, 2, 9437);
        assert_eq!(rate, 786_432_000);
    }

    #[test]
    fn test_pll_count() {
        // RK3588 应该有 9 个 PLL
        assert_eq!(RK3588_PLL_CLOCKS.len(), PllId::_Len as usize);
        assert_eq!(RK3588_PLL_CLOCKS.len(), 9);
    }

    #[test]
    fn test_pll_ids() {
        // 验证 PLL ID 顺序
        assert_eq!(PllId::B0PLL as usize, 0);
        assert_eq!(PllId::B1PLL as usize, 1);
        assert_eq!(PllId::LPLL as usize, 2);
        assert_eq!(PllId::CPLL as usize, 3);
        assert_eq!(PllId::GPLL as usize, 4);
        assert_eq!(PllId::NPLL as usize, 5);
        assert_eq!(PllId::V0PLL as usize, 6);
        assert_eq!(PllId::AUPLL as usize, 7);
        assert_eq!(PllId::PPLL as usize, 8);
    }

    #[test]
    fn test_pll_names() {
        assert_eq!(PllId::GPLL.name(), "GPLL");
        assert_eq!(PllId::CPLL.name(), "CPLL");
        assert_eq!(PllId::NPLL.name(), "NPLL");
    }

    #[test]
    fn test_pll_default_rates() {
        assert_eq!(PllId::GPLL.default_rate(), Some(GPLL_HZ as u64));
        assert_eq!(PllId::CPLL.default_rate(), Some(CPLL_HZ as u64));
        assert_eq!(PllId::NPLL.default_rate(), Some(NPLL_HZ as u64));
    }

    #[test]
    fn test_pll_config_offsets() {
        // 验证关键 PLL 的寄存器偏移
        let gpll = get_pll(PllId::GPLL);
        assert_eq!(gpll.con_offset, pll_con(112));
        assert_eq!(gpll.mode_offset, RK3588_MODE_CON0);
        assert_eq!(gpll.mode_shift, 2);

        let cpll = get_pll(PllId::CPLL);
        assert_eq!(cpll.con_offset, pll_con(104));
        assert_eq!(cpll.mode_offset, RK3588_MODE_CON0);
        assert_eq!(cpll.mode_shift, 8);
    }
}
