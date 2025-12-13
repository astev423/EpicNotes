use egui::{Button, Color32, RichText};

use crate::{draw_gui::Canvas, notes_gui::Notes};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EpicNotesApp {
    pub notes: Notes,
    pub canvas: Canvas,
    pub mode: GuiMode,
    pub dark_mode: bool,
}

// Serde needs this to be serializable and deserializable for notes app
#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum GuiMode {
    Notes,
    Drawing,
}

impl Default for EpicNotesApp {
    // Instantiate struct then transfer ownership to caller
    fn default() -> Self {
        Self {
            notes: Notes::default(),
            canvas: Canvas::default(),
            mode: GuiMode::Notes,
            dark_mode: true,
        }
    }
}

impl EpicNotesApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn header(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let title = RichText::new("Epic Notes")
            .size(30.0)
            .color(Color32::RED)
            .strong()
            .background_color(Color32::DARK_GRAY);

        ui.horizontal(|ui| {
            ui.heading(title);
            self.mode_buttons(ctx, ui);
        });
        ui.add_space(10.0);
    }

    pub fn mode_buttons(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let theme_btn = Button::new("Toggle dark mode");
        if ui.add_sized([35.0, 35.0], theme_btn).clicked() {
            println!("toggline dark mode");
            self.toggle_dark_mode(ctx);
        }

        let mut draw_btn = Button::new("Drawing mode");
        let notes_btn = Button::new("Notes mode").fill(if self.mode == GuiMode::Notes {
            draw_btn = draw_btn.fill(Color32::DARK_GRAY);
            Color32::RED
        } else {
            draw_btn = draw_btn.fill(Color32::RED);
            Color32::DARK_GRAY
        });

        let notes_response = ui.add_sized([35.0, 35.0], notes_btn);
        if notes_response.clicked() {
            self.mode = GuiMode::Notes;
        }
        let drawing_response = ui.add_sized([35.0, 35.0], draw_btn);
        if drawing_response.clicked() {
            self.mode = GuiMode::Drawing;
        };
    }

    pub fn toggle_dark_mode(&mut self, ctx: &egui::Context) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::light());
            self.dark_mode = false;
        } else {
            ctx.set_visuals(egui::Visuals::dark());
            self.dark_mode = true;
        }
    }
}

impl eframe::App for EpicNotesApp {
    // Called by the framework to save state before shutdown.
    // OS sends a call to closedown app, this responds to that call by serializing then exits
    // dyn here because eframe::Storage is a trait
    fn save(self: &mut EpicNotesApp, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create default panel, then call this closure to run the rest
        egui::CentralPanel::default().show(ctx, |ui| {
            self.header(ctx, ui);
            match self.mode {
                GuiMode::Notes => self.notes.display_notes_gui(ui),
                GuiMode::Drawing => self.canvas.display_draw_gui(ui),
            };
        });
    }
}
