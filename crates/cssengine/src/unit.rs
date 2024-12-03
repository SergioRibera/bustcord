mod color;
mod cursor;
mod px;
mod text;

use csscolorparser::Color;

pub use color::NAMED_COLORS;

#[cfg(feature = "tailwind_colors")]
pub(crate) use color::{TAILWIND_COLORS, TAILWIND_NAME_COLORS};

pub use cursor::*;
pub use px::*;
pub use text::{Style as TextStyle, TextOverflow, Weight};

/// Used for automatically computed values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Auto;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BorderDef {
    pub width: Option<PxPct>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoxShadow {
    pub blur_radius: PxPct,
    pub color: Color,
    pub spread: PxPct,
    pub h_offset: PxPct,
    pub v_offset: PxPct,
}

impl Default for BoxShadow {
    fn default() -> Self {
        Self {
            blur_radius: PxPct::Px(0.),
            color: Color::from_rgba8(0, 0, 0, 255),
            spread: PxPct::Px(0.),
            h_offset: PxPct::Px(0.),
            v_offset: PxPct::Px(0.),
        }
    }
}
