mod cursor;
mod px;
mod text;

use color::DynamicColor;

pub use cursor::*;
pub use px::*;
pub use text::{Style as TextStyle, TextOverflow, Weight};

/// Used for automatically computed values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Auto;

#[derive(Clone, Debug, Default)]
pub struct BorderDef {
    pub width: Option<PxPct>,
    pub color: Option<DynamicColor>,
}

#[derive(Debug, Clone, Copy)]
pub struct BoxShadow {
    pub blur_radius: PxPct,
    pub color: DynamicColor,
    pub spread: PxPct,
    pub h_offset: PxPct,
    pub v_offset: PxPct,
}

impl Default for BoxShadow {
    fn default() -> Self {
        Self {
            blur_radius: PxPct::Px(0.),
            color: color::parse_color("#000").unwrap(),
            spread: PxPct::Px(0.),
            h_offset: PxPct::Px(0.),
            v_offset: PxPct::Px(0.),
        }
    }
}
