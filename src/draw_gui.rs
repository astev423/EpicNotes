use egui::{Button, Color32, Context, Id, Modal, Pos2, Ui, color_picker::Alpha};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Canvas {
    paint_color: Color32,
    positions_colored: Vec<(Pos2, f32, Color32)>,
    brush_size: f32,
    show_clear_modal: bool,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            paint_color: egui::Color32::WHITE,
            positions_colored: Vec::new(),
            brush_size: 18.0,
            show_clear_modal: false,
        }
    }
}

impl Canvas {
    pub fn display_draw_gui(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.horizontal(|ui| {
            // Brush size slider
            ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=50.0).text("Brush size"));

            // Color picker button
            egui::color_picker::color_edit_button_srgba(ui, &mut self.paint_color, Alpha::Opaque);

            // Clear button
            if ui.button("Clear canvas").clicked() {
                self.show_clear_modal = true;
            }
            if self.show_clear_modal {
                let modal = Modal::new(Id::new("modal"));
                modal.show(ctx, |ui| {
                    ui.label("Are you sure?");
                    ui.vertical_centered_justified(|ui| {
                        let yes_button = Button::new("Yes").fill(Color32::DARK_RED);
                        let yes_res = ui.add(yes_button);
                        if yes_res.clicked() {
                            self.positions_colored.clear();
                            self.show_clear_modal = false;
                        }
                        let no_button = Button::new("No").fill(Color32::DARK_RED);
                        let no_res = ui.add(no_button);
                        if no_res.clicked() {
                            self.show_clear_modal = false;
                        }
                    })
                });
            }
        });

        self.show_canvas(ui);
    }

    /// This makes a single line for as long as user drags, adding new positions every moment
    fn show_canvas(&mut self, ui: &mut Ui) {
        // Get region to paint on, input size of it and what response responds to
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        // Background
        let rect = response.rect;
        painter.rect_filled(rect, 0.0, egui::Color32::from_gray(20));

        // Push all positions drawn and color of them into vector
        if response.dragged() {
            if let Some(cur_mouse_position) = response.interact_pointer_pos() {
                self.positions_colored.push((
                    cur_mouse_position.clamp(rect.min, rect.max),
                    self.brush_size,
                    self.paint_color,
                ));
            }
        }

        // Draw all strokes
        for (position, brush_size, color) in self.positions_colored.iter() {
            painter.circle_filled(position.clone(), *brush_size, color.clone());
        }
    }
}
