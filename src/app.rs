use std::fs::write;
use std::io::Error;
use std::{f32::INFINITY, vec};

use egui::{Button, TextEdit, Ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EpicNotesApp {
    notes: Vec<String>,
    selected_notepage: i32,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for EpicNotesApp {
    fn default() -> Self {
        Self {
            notes: vec![String::from("Hello")],
            value: 2.7,
            selected_notepage: 0,
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

    fn export_notes(&self, filename: &str) -> Result<(), Error> {
        write(filename, &self.notes[0])?;
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
            ui.heading("Epic Notes");
            select_notes(self, ui);
            notes_0(self, ui);
        });
    }
}

pub fn select_notes(app: &mut EpicNotesApp, ui: &mut Ui) {
    let options = ["Option A", "Option B", "Option C"];
    let mut b = String::from("selectable value");

    egui::ComboBox::from_label("Choose your notes")
        .selected_text("Selected text")
        .show_ui(ui, |ui| {
            for option in options {
                ui.selectable_value(&mut b, option.to_owned(), option);
            }
        });
}

pub fn notes_0(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    let response = ui.add(Button::new("Clear text"));
    if response.clicked() {
        let result = app.export_notes("/home/alex/projs/rust/epic_notes/src/notes.txt");
        match result {
            Ok(_) => println!("Saved notes to notes.txt"),
            _ => println!("Major FAIL to open file"),
        }

        app.notes[0].clear();
    }
    // ui.text_edit_multiline(notes) is just a convenience for this below
    let notes = &mut app.notes[0];
    ui.add(
        TextEdit::multiline(notes)
            .desired_width(INFINITY)
            .desired_rows(999),
    );
}
