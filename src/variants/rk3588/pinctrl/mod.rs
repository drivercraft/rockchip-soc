//! RK3588 Pinctrl 模块
//!
//! 提供引脚复用和引脚配置功能。

mod iomux;
mod pinconf_regs;
mod pinctrl;

pub use pinctrl::Pinctrl;
