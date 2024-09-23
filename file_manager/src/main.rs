mod file_manager;

use file_manager::FileManager;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("File Manager", options, Box::new(FileManager::default()))
}