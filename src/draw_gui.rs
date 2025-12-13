use egui::{Ui, color_picker::Alpha};

use crate::app::EpicNotesApp;

pub fn display_draw_gui(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        // Brush size slider
        ui.add(egui::Slider::new(&mut app.brush_size, 1.0..=50.0).text("Brush size"));

        // Color picker button
        egui::color_picker::color_edit_button_srgba(ui, &mut app.paint_color, Alpha::Opaque);

        // Clear button
        if ui.button("Clear canvas").clicked() {
            app.paint_lines.clear();
        }
    });

    show_canvas(app, ui);
}

fn show_canvas(app: &mut EpicNotesApp, ui: &mut Ui) {
    // Use all remaining space as the canvas (or pick a fixed size instead)
    let desired_size = ui.available_size();
    let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::drag());

    let rect = response.rect;

    // Background
    painter.rect_filled(rect, 0.0, egui::Color32::from_gray(20));

    // Start a new stroke when drag starts
    if response.drag_started() {
        app.paint_lines.push(Vec::new());
    }

    // While dragging, push the current pointer position into the last stroke
    if response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            if let Some(line) = app.paint_lines.last_mut() {
                line.push(pos.clamp(rect.min, rect.max));
            }
        }
    }

    // Draw all strokes
    let stroke = egui::Stroke::new(app.brush_size, app.paint_color);
    for line in &app.paint_lines {
        for points in line.windows(2) {
            painter.line_segment([points[0], points[1]], stroke);
        }
    }
}
