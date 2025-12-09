use egui::{Ui, Window};

use crate::app::EpicNotesApp;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Canvas {
    pixel_grid_2d: Vec<Vec<bool>>,
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas {
            pixel_grid_2d: vec![Vec::new()],
        }
    }
}

fn show_canvas(app: &mut EpicNotesApp, ui: &mut Ui) {
    let wind = Window::new("canvas window").auto_sized();
    ui.add(wind);
}

pub fn display_draw_gui(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add(egui::Slider::new(&mut app.notes_font_size, 5.0..=150.0).text("Brush size"));
        // Add color button here
        if ui.button("Clear canvas").clicked() {
            println!("clearing canvas");
        };
    });
    show_canvas(app, ui);
}
