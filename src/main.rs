use eframe::NativeOptions;
use epic_notes::app::EpicNotesApp;

fn main() {
    let _ = eframe::run_native(
        "eframe template",
        NativeOptions::default(),
        // cc provides content which stores state, can do TemplateApp::default() to not save anything
        // App run_native needs closure or something
        Box::new(|cc| Ok(Box::new(EpicNotesApp::new(cc)))),
    );
}
