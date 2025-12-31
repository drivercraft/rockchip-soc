pub mod pll;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl ClkId {
    /// 获取时钟 ID 的数值表示
    pub const fn value(&self) -> u64 {
        self.0
    }

    pub const fn new(value: u64) -> Self {
        ClkId(value)
    }
}
