use crate::notes_gui::display_gui;
use std::fs::write;
use std::io::Error;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EpicNotesApp {
    pub notepage_titles: Vec<String>,
    pub notepage_contents: Vec<String>,
    pub selected_notepage: i32,
}

impl Default for EpicNotesApp {
    // Instantiate struct then transfer ownership to caller
    fn default() -> Self {
        Self {
            notepage_titles: vec![String::from("Page one")],
            notepage_contents: vec![String::from("")],
            selected_notepage: 1,
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

    pub fn export_notes(&self) -> Result<(), Error> {
        let path = format!(
            "/home/alex/projs/rust/epic_notes/notes/{}.txt",
            &self.notepage_titles[0]
        );
        write(path, &self.notepage_contents[0])?;
        Ok(())
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
            display_gui(self, ui);
        });
    }
}
