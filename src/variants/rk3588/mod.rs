pub mod cru;
pub mod gpio;
pub mod pinctrl;
pub mod pinmanager;
mod syscon;

// =============================================================================
// 公开导出
// =============================================================================

pub use cru::Cru;
pub use gpio::GpioBank;
pub use pinctrl::Pinctrl;
pub use pinmanager::PinManager;
