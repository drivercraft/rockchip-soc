//! Pinmux 功能类型
//!
//! 定义引脚复用功能选择。

use super::GpioDirection;

bitflags::bitflags! {
    /// IOMUX 配置标志
    ///
    /// 定义引脚复用控制的属性和特性,对应 Rockchip pinctrl 驱动中的 iomux 标志。
    ///
    /// # 标志说明
    ///
    /// - `GPIO_ONLY`: 引脚仅支持 GPIO 模式,不支持复用功能
    /// - `WIDTH_4BIT`: 功能选择位宽为 4 位
    /// - `SOURCE_PMU`: 寄存器位于 PMU (Power Management Unit) 地址空间
    /// - `UNROUTED`: 未路由的引脚(无实际连接)
    /// - `WIDTH_3BIT`: 功能选择位宽为 3 位
    /// - `8WIDTH_2BIT`: 8 个引脚共享 2 位宽度的功能选择
    /// - `WRITABLE_32BIT`: 使用 32 位写操作(而非 16 位)
    /// - `L_SOURCE_PMU`: 低 16 位位于 PMU 地址空间
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Iomux: u8 {
        /// 仅 GPIO 模式(无复用功能)
        const GPIO_ONLY = 1;
        /// 功能选择位宽为 4 位
        const WIDTH_4BIT = 1 << 1;
        /// 寄存器位于 PMU 地址空间
        const SOURCE_PMU = 1 << 2;
        /// 未路由的引脚
        const UNROUTED = 1 << 3;
        /// 功能选择位宽为 3 位
        const WIDTH_3BIT = 1 << 4;
        /// 8 引脚共享 2 位功能选择
        const WIDTH_8_2BIT = 1 << 5;
        /// 使用 32 位写操作
        const WRITABLE_32BIT = 1 << 6;
        /// 低 16 位位于 PMU 地址空间
        const L_SOURCE_PMU = 1 << 7;
    }
}

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
