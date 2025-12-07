use std::fs::write;
use std::io::Error;

use egui::{Button, ScrollArea, TextEdit, Ui, Vec2};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EpicNotesApp {
    notepage_titles: Vec<String>,
    notepage_contents: Vec<String>,
    selected_notepage: i32,
}

impl Default for EpicNotesApp {
    // Instantiate struct then transfer ownership to caller
    fn default() -> Self {
        Self {
            notepage_titles: vec![String::from("Page one")],
            notepage_contents: vec![String::from("")],
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

    fn export_notes(&self) -> Result<(), Error> {
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

fn add_notes(app: &mut EpicNotesApp, ui: &mut Ui) {
    let add_response = ui.button("Click here to add another notepage!");
    if add_response.clicked() {
        app.notepage_titles
            .push(format!("Page {}", app.notepage_titles.len()));
        app.notepage_contents.push(String::new());
    }

    let del_response = ui.button("Click here to delete current notepage!");
    if del_response.clicked() {
        if app.notepage_titles.len() == 1 {
            println!("Can't delete your last page of notes!");
        } else {
            app.notepage_titles.remove(app.selected_notepage as usize);
            app.notepage_contents.remove(app.selected_notepage as usize);
        }
    }
}

fn select_notes(app: &mut EpicNotesApp, ui: &mut Ui) {
    let notes_titles = &app.notepage_titles;
    let mut b = String::from("selectable value");

    egui::ComboBox::from_id_salt("notes_dropdown")
        .selected_text("Selected text")
        .show_ui(ui, |ui| {
            for title in notes_titles {
                ui.selectable_value(&mut b, title.to_owned(), title);
            }
        });
}

fn save_notes(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    let response = ui.add(Button::new("Clear text"));
    if response.clicked() {
        let result = app.export_notes();
        match result {
            Ok(_) => println!("Saved notes to notes.txt"),
            _ => println!("Major FAIL to open file"),
        }

        app.notepage_contents[0].clear();
    }
}

fn show_notepage(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    let notes = &mut app.notepage_contents[app.selected_notepage as usize];

    ui.add_space(10.0);
    // To make things scrollable need to wrap them in ScrollArea
    ScrollArea::vertical().show(ui, |ui| {
        // Size it with availible size for responsiveness
        ui.add_sized(ui.available_size(), TextEdit::multiline(notes));
    });
}

fn display_gui(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    ui.heading("Epic Notes");
    ui.horizontal(|ui| {
        select_notes(app, ui);
        add_notes(app, ui);
    });
    save_notes(app, ui);
    show_notepage(app, ui);
}
