use egui::{Button, Image, Sense, Ui, Widget};

// TODO: from common get Server Information struct with server basic data
// This need the name, image before name on hover, voice chat information,
// voice chat participants
pub fn server(
    ui: &mut Ui,
    image: Option<Image>,
    name: Option<&str>,
    is_vc: bool,
    on_click: impl FnOnce(),
) {
    let button = match (image, name) {
        (None, Some(name)) => Button::new(name),
        (Some(img), None) | (Some(img), Some(_)) => Button::image(img),
        _ => Button::new("+"),
    };

    if ui.add(button).clicked() {
        on_click();
    }

    if is_vc {
        todo!("Overlay icon");
    }
}

pub struct Servers;

impl Widget for Servers {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let size = ui.available_size();
        let response = ui.allocate_response(size, Sense::focusable_noninteractive());

        response
    }
}
