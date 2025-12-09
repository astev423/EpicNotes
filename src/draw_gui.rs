use crate::app::EpicNotesApp;

pub fn display_draw_gui(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add(egui::Slider::new(&mut app.notes_font_size, 5.0..=150.0).text("Brush size"));
        // Add color button here
        if ui.button("Clear canvas").clicked() {
            println!("clearing canvas");
        };
    });
    // Add actual canvas here
}
