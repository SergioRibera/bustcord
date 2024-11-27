use egui::Label;

#[derive(Default)]
pub struct MainApp;

impl MainApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for MainApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        layouts::home(
            ctx,
            components::Servers,
            Label::new("Chats"),
            Label::new("Content"),
            None::<Label>,
        );
    }
}
