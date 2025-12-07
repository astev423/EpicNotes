use egui::{Button, ScrollArea, TextEdit, Ui};

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
            // Find new page to go to before deleting current one, and don't try negative index
            let old_page_num = app.selected_notepage;
            if app.selected_notepage == 0 {
                // Len - 1 is normal for index but we -2 since we also subtract an element from vec
                app.selected_notepage = (app.notepage_titles.len() - 2) as i32;
            } else {
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
    let response = ui.add(Button::new("Clear text"));
    if response.clicked() {
        let result = app.export_notes();
        match result {
            Ok(_) => println!("Saved notes to notes.txt"),
            _ => println!("Major FAIL to open file"),
        }

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

pub fn display_gui(app: &mut EpicNotesApp, ui: &mut egui::Ui) {
    ui.heading("Epic Notes");
    ui.horizontal(|ui| {
        select_notes_dropdown(app, ui);
        add_notes(app, ui);
        del_notes(app, ui);
    });
    save_notes(app, ui);
    show_notepage(app, ui);
}
