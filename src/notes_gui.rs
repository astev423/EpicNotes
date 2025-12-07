use egui::{Button, Label, ScrollArea, TextEdit, Ui};

use crate::app::EpicNotesApp;

fn add_notes(app: &mut EpicNotesApp, ui: &mut Ui) {
    let add_response = ui.button("Click here to add another notepage!");
    if add_response.clicked() {
        app.notepage_titles
            .push(format!("Page {}", app.notepage_titles.len() + 1));
        app.notepage_contents.push(String::new());
    }
}

fn del_notes(app: &mut EpicNotesApp, ui: &mut Ui) {
    let del_response = ui.button("Click here to delete current notepage!");
    if del_response.clicked() {
        if app.notepage_titles.len() == 1 {
            println!("Can't delete your last page of notes!");
        } else {
            // Remove notes from vec, but change notes we are viewing first otherwise out of bounds read
            let old_page_num = app.selected_notepage;
            // Go to below notes, unless we are already at bottom
            if app.selected_notepage != 0 {
                app.selected_notepage -= 1;
            }
            app.notepage_titles.remove(old_page_num as usize);
            app.notepage_contents.remove(old_page_num as usize);
        }
    }
}

fn select_notes_dropdown(app: &mut EpicNotesApp, ui: &mut Ui) {
    let notes_titles = &app.notepage_titles;
    let mut b = String::from("selectable value");

    egui::ComboBox::from_id_salt("notes_dropdown")
        .selected_text(&app.notepage_titles[app.selected_notepage as usize])
        .show_ui(ui, |ui| {
            for (index, title) in notes_titles.iter().enumerate() {
                // Make button and respond if it gets clicked
                if ui
                    .selectable_value(&mut b, title.to_owned(), title)
                    .clicked()
                {
                    app.selected_notepage = index as i32;
                }
            }
        });
}

fn save_notes(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    let response = ui.add(Button::new("Save notes"));
    if response.clicked() {
        let result = app.export_notes();
        match result {
            Ok(_) => println!("Saved notes to notes.txt"),
            _ => println!("Major FAIL to open file"),
        }
    }
}

fn clear_notes(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    let response = ui.add(Button::new("Clear notes"));
    if response.clicked() {
        app.notepage_contents[app.selected_notepage as usize].clear();
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

fn change_notes_title(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add(Label::new("Change your title here: "));
        ui.add(
            TextEdit::singleline(&mut app.notepage_titles[app.selected_notepage as usize])
                .char_limit(40),
        );
    });
}

pub fn display_gui(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    ui.heading("Epic Notes");
    // For dealing with list of notes
    ui.horizontal(|ui| {
        select_notes_dropdown(app, ui);
        add_notes(app, ui);
        del_notes(app, ui);
    });

    // For dealing with current note page
    change_notes_title(app, ui);
    ui.horizontal(|ui| {
        save_notes(app, ui);
        clear_notes(app, ui);
    });
    show_notepage(app, ui);
}
