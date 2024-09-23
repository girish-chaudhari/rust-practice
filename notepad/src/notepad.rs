use eframe::{egui, epi};

pub struct NotepadApp {
    text: String,
}

impl Default for NotepadApp {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}

impl epi::App for NotepadApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Notepad");
            ui.text_edit_multiline(&mut self.text);
            // ui.style().
        });
    }

    fn name(&self) -> &str {
        "Notepad"
    }
}