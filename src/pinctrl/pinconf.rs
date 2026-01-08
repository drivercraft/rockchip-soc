//! Pinconf 配置类型
//!
//! 定义引脚电气属性配置，包括上下拉、驱动强度等。

/// 引脚上下拉配置
///
/// 定义引脚的上下拉电阻配置。
///
/// # 示例
///
/// ```
/// use rockchip_soc::pinctrl::Pull;
///
/// // 配置为上拉
/// let pull = Pull::PullUp;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Pull {
    /// 禁用上下拉
    Disabled = 0,

    /// 上拉
    PullUp = 1,

    /// 下拉
    PullDown = 2,
}

/// 驱动强度配置（mA）
///
/// 定义引脚输出驱动强度，单位为毫安。
///
/// # 示例
///
/// ```
/// use rockchip_soc::pinctrl::DriveStrength;
///
/// // 配置为 8mA 驱动强度
/// let drive = DriveStrength::Ma8;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum DriveStrength {
    /// 2 mA
    Ma2 = 0,

    /// 4 mA
    Ma4 = 1,

    /// 8 mA
    Ma8 = 2,

    /// 12 mA
    Ma12 = 3,
}

/// 完整的引脚配置
///
/// 组合了引脚功能、上下拉和驱动强度的完整配置。
///
/// # 示例
///
/// ```
/// use rockchip_soc::pinctrl::{PinConfig, Function, Pull, DriveStrength};
///
/// // 创建基础配置
/// let config = PinConfig::new(Function::Alt1);
///
/// // 使用 builder 模式添加配置
/// let config = PinConfig::new(Function::Alt1)
///     .with_pull(Pull::PullUp)
///     .with_drive(DriveStrength::Ma8);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinConfig {
    /// 引脚功能
    pub function: super::Function,

    /// 可选的上下拉配置
    pub pull: Option<Pull>,

    /// 可选的驱动强度配置
    pub drive: Option<DriveStrength>,
}

impl PinConfig {
    /// 创建新的引脚配置
    ///
    /// # 参数
    ///
    /// * `function` - 引脚功能
    pub const fn new(function: super::Function) -> Self {
        Self {
            function,
            pull: None,
            drive: None,
        }
    }

    /// 设置上下拉配置
    ///
    /// # 参数
    ///
    /// * `pull` - 上下拉配置
    pub const fn with_pull(mut self, pull: Pull) -> Self {
        self.pull = Some(pull);
        self
    }

    /// 设置驱动强度配置
    ///
    /// # 参数
    ///
    /// * `drive` - 驱动强度配置
    pub const fn with_drive(mut self, drive: DriveStrength) -> Self {
        self.drive = Some(drive);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinctrl::Function;

    #[test]
    fn test_pin_config_builder() {
        let config = PinConfig::new(Function::Gpio)
            .with_pull(Pull::PullUp)
            .with_drive(DriveStrength::Ma8);

        assert_eq!(config.function, Function::Gpio);
        assert_eq!(config.pull, Some(Pull::PullUp));
        assert_eq!(config.drive, Some(DriveStrength::Ma8));
    }

    #[test]
    fn test_pull_values() {
        assert_eq!(Pull::Disabled as u32, 0);
        assert_eq!(Pull::PullUp as u32, 1);
        assert_eq!(Pull::PullDown as u32, 2);
    }

    #[test]
    fn test_drive_strength_values() {
        assert_eq!(DriveStrength::Ma2 as u32, 0);
        assert_eq!(DriveStrength::Ma4 as u32, 1);
        assert_eq!(DriveStrength::Ma8 as u32, 2);
        assert_eq!(DriveStrength::Ma12 as u32, 3);
    }
}
