/// PLL 类型枚举
///
/// 参考 rockchip_pll_type 定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockchipPllType {
    /// RK3036/3366/3368 类型 PLL
    PllRk3036,
    /// RK3066 类型 PLL
    PllRk3066,
    /// RK3399 类型 PLL
    PllRk3399,
    /// RV1108 类型 PLL
    PllRv1108,
    /// RK3588 类型 PLL
    PllRk3588,
}

/// PLL 速率表项
///
/// 用于描述 PLL 在不同频率下的配置参数
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PllRateTable {
    /// 输出频率 (Hz)
    pub rate: u64,
    /// PLL 特定参数 (根据芯片类型)
    pub params: PllRateParams,
}

/// PLL 速率参数 (根据芯片类型)
#[derive(Debug, Clone, Copy)]
pub enum PllRateParams {
    Normal {
        /// 参考分频系数 (Reference Divider)
        nr: u32,

        /// 反馈分频系数 (Feedback Divider)
        f: u32,

        /// 输出分频系数 (Output Divider)
        no: u32,

        /// 带宽分频系数 (Bandwidth Divider)
        nb: u32,
    },

    /// RK3036/RK3399 类型参数
    Rk3036 {
        /// 反馈分频系数
        fbdiv: u32,
        /// 后分频器 1
        postdiv1: u32,
        /// 参考分频系数
        refdiv: u32,
        /// 后分频器 2
        postdiv2: u32,
        /// 小数分频使能 (0=启用, 1=禁用)
        dsmpd: u32,
        /// 小数分频系数
        frac: u32,
    },

    /// RK3588 类型参数
    Rk3588 {
        /// M 分频系数 (Main Divider)
        m: u32,
        /// P 分频系数 (Pre-divider)
        p: u32,
        /// S 分频系数 (Post-divider)
        s: u32,
        /// K 小数分频系数
        k: u32,
    },
}

impl PllRateTable {}

/// PLL 标志位
///
/// 参考 PLL 相关的标志位定义
pub mod pll_flags {
    /// PLL 需要同步模式
    pub const PLL_SYNC: u32 = 1 << 0;
    /// PLL 使用小数分频
    pub const PLL_FRAC: u32 = 1 << 1;
    /// PLL 使用 4 个后分频器
    pub const PLL_POSTDIV4: u32 = 1 << 2;
    /// PLL 使用 RK3588 类型
    pub const PLL_RK3588: u32 = 1 << 3;
    /// PLL 使用 RK3399 类型
    pub const PLL_RK3399: u32 = 1 << 4;
}

/// Rockchip PLL 时钟结构
#[derive(Debug)]
#[repr(C)]
pub struct PllClock {
    /// 时钟 ID
    pub id: u32,

    /// PLL 控制寄存器偏移量
    pub con_offset: u32,

    /// 模式寄存器偏移量
    pub mode_offset: u32,

    /// 模式位偏移
    pub mode_shift: u32,

    /// 锁定位偏移
    pub lock_shift: u32,

    /// PLL 类型
    pub pll_type: RockchipPllType,

    /// PLL 标志位 (参见 pll_flags 模块)
    pub pll_flags: u32,

    /// PLL 速率表指针
    pub rate_table: Option<&'static PllRateTable>,

    /// 模式掩码
    pub mode_mask: u32,
}

impl PllClock {
    /// 创建新的 PLL 时钟实例
    ///
    /// # 参数
    ///
    /// * `id` - 时钟 ID
    /// * `con_offset` - PLL 控制寄存器偏移
    /// * `mode_offset` - 模式寄存器偏移
    /// * `mode_shift` - 模式位偏移
    /// * `lock_shift` - 锁定位偏移
    /// * `pll_type` - PLL 类型
    /// * `pll_flags` - PLL 标志位
    /// * `rate_table` - 可选的速率表
    /// * `mode_mask` - 模式掩码
    #[must_use]
    pub const fn new(
        id: u32,
        con_offset: u32,
        mode_offset: u32,
        mode_shift: u32,
        lock_shift: u32,
        pll_type: RockchipPllType,
        pll_flags: u32,
        rate_table: Option<&'static PllRateTable>,
        mode_mask: u32,
    ) -> Self {
        Self {
            id,
            con_offset,
            mode_offset,
            mode_shift,
            lock_shift,
            pll_type,
            pll_flags,
            rate_table,
            mode_mask,
        }
    }

    /// 检查 PLL 是否已锁定
    ///
    /// # 参数
    ///
    /// * `base` - CRU 基地址
    ///
    /// # 返回
    ///
    /// 如果 PLL 已锁定返回 `true`,否则返回 `false`
    #[must_use]
    pub fn is_locked(&self, base: usize) -> bool {
        let reg_addr = base + self.con_offset as usize;
        unsafe {
            let reg = reg_addr as *const u32;
            let val = core::ptr::read_volatile(reg);
            (val & (1 << self.lock_shift)) != 0
        }
    }

    /// 获取 PLL 当前模式
    ///
    /// # 参数
    ///
    /// * `base` - CRU 基地址
    ///
    /// # 返回
    ///
    /// 当前模式值
    #[must_use]
    pub fn get_mode(&self, base: usize) -> u32 {
        let reg_addr = base + self.mode_offset as usize;
        unsafe {
            let reg = reg_addr as *const u32;
            let val = core::ptr::read_volatile(reg);
            (val & self.mode_mask) >> self.mode_shift
        }
    }

    /// 设置 PLL 模式
    ///
    /// # 参数
    ///
    /// * `base` - CRU 基地址
    /// * `mode` - 要设置的模式值
    pub fn set_mode(&self, base: usize, mode: u32) {
        let reg_addr = base + self.mode_offset as usize;
        unsafe {
            let reg = reg_addr as *mut u32;
            let current = core::ptr::read_volatile(reg);
            let new_val =
                (current & !self.mode_mask) | ((mode << self.mode_shift) & self.mode_mask);
            core::ptr::write_volatile(reg, new_val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 示例: 创建 RK3588 GPLL 实例
    #[test]
    fn test_pll_creation() {
        // 创建 RK3588 类型的速率表
        static RK3588_RATE: PllRateTable = PllRateTable::rk3588(
            1188 * 1_000_000, // 1188 MHz
            1,                // nr
            99,               // nf
            1,                // no
            0,                // nb
            99,               // m
            3,                // p
            1,                // s
            0,                // k
        );

        // 创建 GPLL 实例
        let gpll = PllClock::new(
            1,                          // id: GPLL
            0x0000,                     // con_offset: PLL_CON0
            0x0100,                     // mode_offset: MODE_CON0
            8,                          // mode_shift
            10,                         // lock_shift
            RockchipPllType::PllRk3588, // pll_type
            pll_flags::PLL_RK3588,      // pll_flags
            Some(&RK3588_RATE),         // rate_table
            0x3,                        // mode_mask: 2 bits
        );

        assert_eq!(gpll.id, 1);
        assert_eq!(gpll.con_offset, 0x0000);
        assert_eq!(gpll.pll_type, RockchipPllType::PllRk3588);
    }

    #[test]
    fn test_pll_rate_table_rk3036() {
        // 测试 RK3036 类型速率表
        let rate = PllRateTable::rk3036(
            1188 * 1_000_000, // rate
            1,                // nr
            99,               // nf
            1,                // no
            0,                // nb
            99,               // fbdiv
            1,                // postdiv1
            1,                // refdiv
            1,                // postdiv2
            1,                // dsmpd
            0,                // frac
        );

        assert_eq!(rate.rate, 1188 * 1_000_000);

        // 验证参数类型
        match rate.params {
            PllRateParams::Rk3036 { fbdiv, .. } => {
                assert_eq!(fbdiv, 99);
            }
            _ => panic!("Expected Rk3036 params"),
        }
    }

    #[test]
    fn test_pll_rate_table_rk3588() {
        // 测试 RK3588 类型速率表
        let rate = PllRateTable::rk3588(
            1188 * 1_000_000, // rate
            1,                // nr
            99,               // nf
            1,                // no
            0,                // nb
            99,               // m
            3,                // p
            1,                // s
            0,                // k
        );

        assert_eq!(rate.rate, 1188 * 1_000_000);

        // 验证参数类型
        match rate.params {
            PllRateParams::Rk3588 { m, p, s, k } => {
                assert_eq!(m, 99);
                assert_eq!(p, 3);
                assert_eq!(s, 1);
                assert_eq!(k, 0);
            }
            _ => panic!("Expected Rk3588 params"),
        }
    }

    #[test]
    fn test_pll_type_values() {
        // 验证枚举值的整型对应关系
        assert_eq!(RockchipPllType::PllRk3036 as u32, 0);
        assert_eq!(RockchipPllType::PllRk3066 as u32, 1);
        assert_eq!(RockchipPllType::PllRk3399 as u32, 2);
        assert_eq!(RockchipPllType::PllRv1108 as u32, 3);
        assert_eq!(RockchipPllType::PllRk3588 as u32, 4);
    }

    #[test]
    fn test_pll_flags() {
        // 验证标志位定义
        assert_eq!(pll_flags::PLL_SYNC, 1 << 0);
        assert_eq!(pll_flags::PLL_FRAC, 1 << 1);
        assert_eq!(pll_flags::PLL_POSTDIV4, 1 << 2);
        assert_eq!(pll_flags::PLL_RK3588, 1 << 3);
        assert_eq!(pll_flags::PLL_RK3399, 1 << 4);
    }
}
