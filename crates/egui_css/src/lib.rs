use std::sync::LazyLock;

use egui::mutex::Mutex;

mod apply;
mod widget;

pub use cssengine::StyleSheet;
pub use widget::{StyledWidget, StyledWidgetExt};

pub(crate) static GLOBAL_STYLES: LazyLock<Mutex<Option<StyleSheet>>> =
    LazyLock::new(|| Mutex::new(None));

pub fn change_style<'a>(style: StyleSheet<'a>) {
    GLOBAL_STYLES
        .lock()
        .replace(unsafe { core::mem::transmute::<StyleSheet, StyleSheet<'static>>(style) });
}
