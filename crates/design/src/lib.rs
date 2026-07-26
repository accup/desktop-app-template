//! GUI の規約とプリミティブを定めるデザインシステムレイヤー。

mod font_definitions;
mod font_license;
mod font_licenses;
mod license_modal_width;
mod serif_font_family;

pub use font_definitions::font_definitions;
pub use font_license::FontLicense;
pub use font_licenses::FONT_LICENSES;
pub use license_modal_width::LICENSE_MODAL_WIDTH;
pub use serif_font_family::serif_font_family;
