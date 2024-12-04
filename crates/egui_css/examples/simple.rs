use egui::{Button, Label};
use egui_css::{StyleSheet, StyledWidgetExt};

fn main() -> eframe::Result {
    eframe::run_native(
        "egui_css simple",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MainApp::new(cc)))),
    )
}

#[derive(Default)]
pub struct MainApp {
    age: u32,
}

impl MainApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        egui_css::change_style(StyleSheet::from_css(
            r#"
            .header {
              color: purple;
              padding: 20px;
              background-color: black;
            }

            #counter {
              color: green;
              background-color: white;
              padding: 20px;
              margin: 20px;
            }
        "#,
        ));

        Default::default()
    }
}

impl eframe::App for MainApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(Label::new("My egui Application").styled().class(".header"));
                if ui
                    .add(Button::new("Increment").styled().css_id("counter"))
                    .clicked()
                {
                    self.age += 1;
                }
                ui.add(
                    Label::new(format!("age {}", self.age))
                        .styled()
                        .class(".text-orange-700"),
                );
                ui.add(Label::new("No styled"));
            })
        });
    }
}
