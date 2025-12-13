use egui::{Pos2, Ui, color_picker::Alpha};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Canvas {
    pub paint_color: egui::Color32,
    pub paint_lines: Vec<Vec<Pos2>>,
    pub brush_size: f32,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            paint_color: egui::Color32::WHITE,
            paint_lines: Vec::new(),
            brush_size: 18.0,
        }
    }
}

impl Canvas {
    pub fn display_draw_gui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Brush size slider
            ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=50.0).text("Brush size"));

            // Color picker button
            egui::color_picker::color_edit_button_srgba(ui, &mut self.paint_color, Alpha::Opaque);

            // Clear button
            if ui.button("Clear canvas").clicked() {
                self.paint_lines.clear();
            }
        });

        self.show_canvas(ui);
    }

    fn show_canvas(&mut self, ui: &mut Ui) {
        // Use all remaining space as the canvas (or pick a fixed size instead)
        let desired_size = ui.available_size();
        let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::drag());

        let rect = response.rect;

        // Background
        painter.rect_filled(rect, 0.0, egui::Color32::from_gray(20));

        // Start a new stroke when drag starts
        if response.drag_started() {
            self.paint_lines.push(Vec::new());
        }

        // While dragging, push the current pointer position into the last stroke
        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(line) = self.paint_lines.last_mut() {
                    line.push(pos.clamp(rect.min, rect.max));
                }
            }
        }

        // Draw all strokes
        let stroke = egui::Stroke::new(self.brush_size, self.paint_color);
        for line in self.paint_lines.iter() {
            for points in line.windows(2) {
                painter.line_segment([points[0], points[1]], stroke);
            }
        }
    }
}
