const ICON: &'static [u8] = include_bytes!("../assets/icon-256.png");

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(eframe::icon_data::from_png_bytes(ICON).expect("Failed to load icon")),
        ..Default::default()
    };
    eframe::run_native(
        "Bustcord",
        native_options,
        Box::new(|cc| Ok(Box::new(bustcord::MainApp::new(cc)))),
    )
}
