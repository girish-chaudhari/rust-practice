use eframe::{egui, epi};
use egui_extras::RetainedImage;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileManager {
    pub current_dir: PathBuf,
    pub files: Vec<PathBuf>,
    pub folder_icon: RetainedImage,
    pub file_icon: RetainedImage,
    pub new_file_name: String,
    pub new_folder_name: String,
    pub rename_file_name: String,
    pub search_query: String,
    pub preview_image: Option<RetainedImage>,  // For image previews
}

impl Default for FileManager {
    fn default() -> Self {
        // Load PNG icons for folders and files
        let folder_icon = RetainedImage::from_image_bytes("folder_icon", include_bytes!("icons/folder.png")).unwrap();
        let file_icon = RetainedImage::from_image_bytes("file_icon", include_bytes!("icons/file.png")).unwrap();
        Self {
            current_dir: PathBuf::from("."),
            files: vec![],
            folder_icon,
            file_icon,
            new_file_name: String::new(),
            new_folder_name: String::new(),
            rename_file_name: String::new(),
            search_query: String::new(),
            preview_image: None,
        }
    }
}

impl epi::App for FileManager {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        // Header panel
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("⬅").clicked() {
                    self.go_up();
                }
                ui.label("Current Directory:");
                ui.label(self.current_dir.to_str().unwrap());
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.text_edit_singleline(&mut self.search_query);
                    ui.label("Search:");
                });
            });
        });

        // Sidebar panel
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            ui.heading("Navigation");
            if ui.button("Home").clicked() {
                self.change_directory("/");
            }
            if ui.button("Documents").clicked() {
                self.change_directory("/path/to/documents");
            }
            if ui.button("Downloads").clicked() {
                self.change_directory("/path/to/downloads");
            }
            if ui.button("Pictures").clicked() {
                self.change_directory("/path/to/pictures");
            }
        });

        // Central panel for files and grid view
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Files");
            let files_to_display = self.files.clone(); // Clone the file list to avoid borrow conflicts

            // Show the files in a grid layout
            let grid = egui::Grid::new("file_grid").spacing(egui::vec2(20.0, 20.0));
            grid.show(ui, |ui| {
                for file in files_to_display {
                    let file_path = file.clone();
                    let file_name = file.file_name().unwrap().to_str().unwrap();

                    ui.vertical(|ui| {
                        if file.is_dir() {
                            // Display folder icon and handle click
                            if self.folder_icon.show(ui).clicked() {
                                self.change_directory(file_path.to_str().unwrap());
                            }
                        } else {
                            // Display file icon or preview image if it's an image file
                            if self.is_image_file(file_name) {
                                if let Some(preview_image) = &self.preview_image {
                                    if preview_image.show(ui).clicked() {
                                        self.preview_image = Some(self.load_image(&file_path));
                                    }
                                } else {
                                    if self.file_icon.show(ui).clicked() {
                                        // Action for non-image files
                                    }
                                }
                            } else {
                                if self.file_icon.show(ui).clicked() {
                                    // Action for non-image files
                                }
                            }
                        }
                        ui.label(file_name);
                    });

                    if ui.input().key_pressed(egui::Key::Enter) {
                        // Trigger context menu or options for the file on Enter
                    }

                    ui.end_row();  // Move to the next row in the grid
                }
            });
        });

        // Footer panel for creating and renaming files/folders
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("New File:");
                ui.text_edit_singleline(&mut self.new_file_name);
                if ui.button("Create").clicked() {
                    self.create_file();
                }
                ui.label("New Folder:");
                ui.text_edit_singleline(&mut self.new_folder_name);
                if ui.button("Create").clicked() {
                    self.create_folder();
                }
                ui.label("Rename:");
                ui.text_edit_singleline(&mut self.rename_file_name);
                if ui.button("Rename").clicked() {
                    self.rename_file();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "File Manager"
    }
}

impl FileManager {
    pub fn list_files(&mut self) {
        self.files.clear();
        let paths = fs::read_dir(&self.current_dir).unwrap();
        for path in paths {
            self.files.push(path.unwrap().path());
        }
    }

    pub fn create_file(&mut self) {
        let path = self.current_dir.join(&self.new_file_name);
        match fs::File::create(&path) {
            Err(why) => eprintln!("Couldn't create {}: {}", path.display(), why),
            Ok(_) => println!("Successfully created {}", path.display()),
        }
        self.new_file_name.clear();
        self.list_files();
    }

    pub fn create_folder(&mut self) {
        let path = self.current_dir.join(&self.new_folder_name);
        match fs::create_dir(&path) {
            Err(why) => eprintln!("Couldn't create folder {}: {}", path.display(), why),
            Ok(_) => println!("Successfully created folder {}", path.display()),
        }
        self.new_folder_name.clear();
        self.list_files();
    }

    pub fn rename_file(&mut self) {
        let old_path = self.current_dir.join(&self.rename_file_name);
        let new_path = self.current_dir.join(&self.new_file_name);
        match fs::rename(&old_path, &new_path) {
            Err(why) => eprintln!("Couldn't rename {}: {}", old_path.display(), why),
            Ok(_) => println!("Successfully renamed {} to {}", old_path.display(), new_path.display()),
        }
        self.rename_file_name.clear();
        self.new_file_name.clear();
        self.list_files();
    }

    pub fn delete_file(&mut self) {
        let path = self.current_dir.join(&self.rename_file_name);
        match fs::remove_file(&path) {
            Err(why) => eprintln!("Couldn't delete {}: {}", path.display(), why),
            Ok(_) => println!("Successfully deleted {}", path.display()),
        }
        self.rename_file_name.clear();
        self.list_files();
    }

    pub fn change_directory(&mut self, directory: &str) {
        let path = Path::new(directory);
        if path.is_dir() {
            self.current_dir = path.to_path_buf();
            self.list_files();
        }
    }

    pub fn go_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.list_files();
        }
    }

    pub fn is_image_file(&self, file_name: &str) -> bool {
        // Check if the file is an image based on its extension
        file_name.ends_with(".png") || file_name.ends_with(".jpg") || file_name.ends_with(".jpeg")
    }

    pub fn load_image(&self, file_path: &PathBuf) -> Option<RetainedImage> {
        // Load the image from the file path
        RetainedImage::from_image_bytes(file_path.to_str().unwrap(), &fs::read(file_path).unwrap()).ok()
    }

    pub fn search_files(&mut self) {
        let current_dir = self.current_dir.clone();
        self.files.clear();
        self.search_in_directory(&current_dir);
    }

    pub fn search_in_directory(&mut self, directory: &PathBuf) {
        let paths = fs::read_dir(directory).unwrap();
        for path in paths {
            let file_path = path.unwrap().path();
            if file_path.is_dir() {
                self.search_in_directory(&file_path);
            } else {
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                if file_name.contains(&self.search_query) {
                    self.files.push(file_path);
                }
            }
        }
    }
}