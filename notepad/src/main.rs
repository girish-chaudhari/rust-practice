mod notepad;

use notepad::NotepadApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(NotepadApp::default()), options);
    Ok(())
}