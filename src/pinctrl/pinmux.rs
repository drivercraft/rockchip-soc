//! Pinmux 功能类型
//!
//! 定义引脚复用功能选择。

use super::{GpioDirection, Pull};

/// 引脚功能选择
///
/// 定义引脚的复用功能。0 表示 GPIO 功能，其他值对应不同外设功能。
///
/// # 示例
///
/// ```
/// use rockchip_soc::pinctrl::Function;
///
/// // GPIO 功能
/// let gpio = Function::Gpio;
///
/// // 功能 1（如 UART0_TX）
/// let alt1 = Function::Alt1;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Function {
    /// GPIO 功能（默认）
    Gpio(GpioDirection),

    /// 功能 1（如 UART、SPI 等）
    Alt1 = 1,

    /// 功能 2
    Alt2 = 2,

    /// 功能 3
    Alt3 = 3,

    /// 功能 4
    Alt4 = 4,

    /// 功能 5-15（扩展的外设功能）
    Alt5 = 5,
    Alt6 = 6,
    Alt7 = 7,
    Alt8 = 8,
    Alt9 = 9,
    Alt10 = 10,
    Alt11 = 11,
    Alt12 = 12,
    Alt13 = 13,
    Alt14 = 14,
    Alt15 = 15,
}

impl Function {
    /// 获取功能的原始数值
    pub const fn num(self) -> u32 {
        match self {
            Function::Gpio(_) => 0,
            Function::Alt1 => 1,
            Function::Alt2 => 2,
            Function::Alt3 => 3,
            Function::Alt4 => 4,
            Function::Alt5 => 5,
            Function::Alt6 => 6,
            Function::Alt7 => 7,
            Function::Alt8 => 8,
            Function::Alt9 => 9,
            Function::Alt10 => 10,
            Function::Alt11 => 11,
            Function::Alt12 => 12,
            Function::Alt13 => 13,
            Function::Alt14 => 14,
            Function::Alt15 => 15,
        }
    }

    /// 从数值创建功能
    ///
    /// 0 返回 GPIO 功能（默认输入方向）
    /// 1-15 返回对应的外设功能
    /// 其他值返回 None
    pub const fn from_num(num: u32) -> Option<Self> {
        match num {
            0 => Some(Function::Gpio(GpioDirection::Input)),
            1 => Some(Function::Alt1),
            2 => Some(Function::Alt2),
            3 => Some(Function::Alt3),
            4 => Some(Function::Alt4),
            5 => Some(Function::Alt5),
            6 => Some(Function::Alt6),
            7 => Some(Function::Alt7),
            8 => Some(Function::Alt8),
            9 => Some(Function::Alt9),
            10 => Some(Function::Alt10),
            11 => Some(Function::Alt11),
            12 => Some(Function::Alt12),
            13 => Some(Function::Alt13),
            14 => Some(Function::Alt14),
            15 => Some(Function::Alt15),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_raw() {
        assert_eq!(Function::Gpio(GpioDirection::Input).num(), 0);
        assert_eq!(Function::Alt1.num(), 1);
        assert_eq!(Function::Alt2.num(), 2);
        assert_eq!(Function::Alt3.num(), 3);
        assert_eq!(Function::Alt4.num(), 4);
    }
}
