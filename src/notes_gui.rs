use egui::{Button, Context, FontId, Id, Label, Modal, ScrollArea, TextEdit, Ui};
use std::fs::write;
use std::io::Error;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Notes {
    notepage_titles: Vec<String>,
    notepage_contents: Vec<String>,
    selected_notepage: i32,
    notes_font_size: f32,
}

impl Default for Notes {
    fn default() -> Self {
        Self {
            notepage_titles: vec![String::from("Page one")],
            notepage_contents: vec![String::from("")],
            selected_notepage: 0,
            notes_font_size: 18.0,
        }
    }
}

impl Notes {
    fn add_notes(&mut self, ui: &mut Ui) {
        let add_response = ui.button("Click here to add another notepage!");
        if add_response.clicked() {
            self.notepage_titles
                .push(format!("Page {}", self.notepage_titles.len() + 1));
            self.notepage_contents.push(String::new());
        }
    }

    fn del_notes(&mut self, ui: &mut Ui) {
        let del_response = ui.button("Click here to delete current notepage!");
        if del_response.clicked() {
            if self.notepage_titles.len() == 1 {
                println!("Can't delete your last page of notes!");
            } else {
                // Remove notes from vec, but change notes we are viewing first otherwise out of bounds read
                let old_page_num = self.selected_notepage;
                // Go to below notes, unless we are already at bottom
                if self.selected_notepage != 0 {
                    self.selected_notepage -= 1;
                }
                self.notepage_titles.remove(old_page_num as usize);
                self.notepage_contents.remove(old_page_num as usize);
            }
        }
    }

    fn select_notes_dropdown(&mut self, ui: &mut Ui) {
        let notes_titles = &self.notepage_titles;
        let mut b = String::from("selectable value");

        egui::ComboBox::from_id_salt("notes_dropdown")
            .selected_text(&self.notepage_titles[self.selected_notepage as usize])
            .show_ui(ui, |ui| {
                for (index, title) in notes_titles.iter().enumerate() {
                    // Make button and respond if it gets clicked
                    if ui
                        .selectable_value(&mut b, title.to_owned(), title)
                        .clicked()
                    {
                        self.selected_notepage = index as i32;
                    }
                }
            });
    }

    fn export_notes(&mut self) -> Result<(), Error> {
        let path = format!(
            "/home/alex/projs/rust/epic_notes/notes/{}.md",
            &self.notepage_titles[self.selected_notepage as usize]
        );
        println!("{}", path);
        write(
            path,
            &self.notepage_contents[self.selected_notepage as usize],
        )?;
        Ok(())
    }

    fn save_notes(&mut self, ui: &mut egui::Ui) {
        let response = ui.add(Button::new("Save notes"));
        if response.clicked() {
            let result = self.export_notes();
            match result {
                Ok(_) => println!("Saved notes to notes folder in current directory"),
                _ => println!("Major FAIL to open file"),
            }
        }
    }

    fn clear_notes(&mut self, ui: &mut egui::Ui) {
        let response = ui.add(Button::new("Clear notes"));
        // confirm first
        if response.clicked() {
            self.notepage_contents[self.selected_notepage as usize].clear();
        }
    }

    fn change_notes_title(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(Label::new("Change your title here: "));
            ui.add(
                TextEdit::singleline(&mut self.notepage_titles[self.selected_notepage as usize])
                    .char_limit(40),
            );
        });
    }

    fn show_notepage(&mut self, ui: &mut egui::Ui) {
        let notes = &mut self.notepage_contents[self.selected_notepage as usize];

        ui.add_space(10.0);
        ui.add(egui::Slider::new(&mut self.notes_font_size, 5.0..=150.0).text("Notes font size"));
        // To make things scrollable need to wrap them in ScrollArea
        ScrollArea::vertical().show(ui, |ui| {
            // Size it with availible size for responsiveness
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(notes).font(FontId::proportional(self.notes_font_size)),
            );
        });
    }

    /// Three lines for documentation, this calls all methods for Notes
    pub fn display_notes_gui(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let modal = Modal::new(Id::new("modal"));
        // modal.show(ctx, |ui| {
        //     ui.label("are you sure?");
        // });

        // For dealing with list of notes
        ui.horizontal(|ui| {
            self.select_notes_dropdown(ui);
            self.add_notes(ui);
            self.del_notes(ui);
        });

        // For dealing with current note page
        self.change_notes_title(ui);
        ui.horizontal(|ui| {
            self.save_notes(ui);
            self.clear_notes(ui);
        });
        self.show_notepage(ui);
    }
}
