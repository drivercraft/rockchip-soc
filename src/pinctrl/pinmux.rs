//! Pinmux 功能类型
//!
//! 定义引脚复用功能选择。

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
    Gpio = 0,

    /// 功能 1（如 UART、SPI 等）
    Alt1 = 1,

    /// 功能 2
    Alt2 = 2,

    /// 功能 3
    Alt3 = 3,

    /// 功能 4
    Alt4 = 4,
}

impl Function {
    /// 从原始值创建 Function
    ///
    /// # 参数
    ///
    /// * `value` - 原始功能值 (0-4)
    ///
    /// # 返回
    ///
    /// 如果 value 在 0-4 范围内，返回 `Some(Function)`，否则返回 `None`
    pub const fn from_raw(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::Gpio),
            1 => Some(Self::Alt1),
            2 => Some(Self::Alt2),
            3 => Some(Self::Alt3),
            4 => Some(Self::Alt4),
            _ => None,
        }
    }

    /// 获取原始功能值
    pub const fn raw(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_values() {
        assert_eq!(Function::Gpio as u32, 0);
        assert_eq!(Function::Alt1 as u32, 1);
        assert_eq!(Function::Alt2 as u32, 2);
        assert_eq!(Function::Alt3 as u32, 3);
        assert_eq!(Function::Alt4 as u32, 4);
    }

    #[test]
    fn test_function_from_raw() {
        assert_eq!(Function::from_raw(0), Some(Function::Gpio));
        assert_eq!(Function::from_raw(1), Some(Function::Alt1));
        assert_eq!(Function::from_raw(4), Some(Function::Alt4));
        assert_eq!(Function::from_raw(5), None);
    }

    #[test]
    fn test_function_raw() {
        assert_eq!(Function::Gpio.raw(), 0);
        assert_eq!(Function::Alt1.raw(), 1);
        assert_eq!(Function::Alt2.raw(), 2);
        assert_eq!(Function::Alt3.raw(), 3);
        assert_eq!(Function::Alt4.raw(), 4);
    }
}
