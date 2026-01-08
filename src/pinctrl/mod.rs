//! Pinctrl 通用类型定义
//!
//! 提供跨芯片的引脚控制抽象，包括引脚标识、配置类型和错误处理。

use core::fmt;

pub mod id;
mod pinconf;
mod pinmux;

pub use id::{BankId, PinId};
pub use pinconf::{DriveStrength, PinConfig, Pull};
pub use pinmux::Function;

// 重新导出所有 GPIO 常量
pub use id::*;

/// Pinctrl 错误类型
#[derive(Debug)]
pub enum PinctrlError {
    /// 无效的引脚 ID
    InvalidPinId(u32),

    /// 引脚不支持该功能
    InvalidFunction,

    /// 无效的引脚配置
    InvalidConfig,
}

impl fmt::Display for PinctrlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPinId(id) => write!(f, "无效的引脚 ID: {}", id),
            Self::InvalidFunction => write!(f, "引脚不支持该功能"),
            Self::InvalidConfig => write!(f, "无效的引脚配置"),
        }
    }
}

/// Pinctrl 操作 Result 类型
pub type PinctrlResult<T> = core::result::Result<T, PinctrlError>;
