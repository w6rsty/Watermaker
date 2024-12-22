mod home_page;
mod file_select_page;
mod edit_page;
mod output_page;

use home_page::HomePage;
use file_select_page::FileSelectPage;
use edit_page::EditPage;
use output_page::OutputPage;

use std::collections::HashMap;
use eframe::egui;

pub struct AppContext {
    current_page: String,
    pages: HashMap<String, Box<dyn Page>>,
    storage: AppStorage,
}

struct AppStorage {
    pub pdf_files: Vec<String>,
    pub target_dir: String,

    pub text: String,
    pub color: egui::Color32,
    pub position: egui::Vec2,
    pub rotation: f32, // in degrees
    pub size: f32,
}

impl Default for AppContext {
    fn default() -> Self {
        let mut pages: HashMap<String, Box<dyn Page>> = HashMap::new();
        pages.insert("HomePage".to_string(), Box::new(HomePage));
        pages.insert("FileSelectPage".to_string(), Box::new(FileSelectPage));
        pages.insert("EditPage".to_string(), Box::new(EditPage));
        pages.insert("OutputPage".to_string(), Box::new(OutputPage));

        Self {
            current_page: "HomePage".to_string(),
            pages,
            storage: AppStorage {
                pdf_files: Vec::new(),
                target_dir: String::new(),
                text: "Confidential".to_string(),
                color: egui::Color32::LIGHT_GRAY,
                position: egui::vec2(0.15, 0.7),
                rotation: -50.0,
                size: 25.0,
            },
        }
    }
}

impl AppContext {
    pub fn route_to(&mut self, page: &str) {
        if self.pages.contains_key(page) {
            self.current_page = page.to_owned();
        } else {
            log::error!("Not registered page `{}`", page);
        }
    }

    pub fn handle_input(&mut self, ctx: &egui::Context) -> &mut AppContext {
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files: Vec<String> = ctx.input(|i| i.raw.dropped_files.clone())
                .iter()
                .filter_map(|file| {
                    if let Some(path) = &file.path {
                        if path.extension().and_then(|ext| ext.to_str()) == Some("pdf") {
                            return Some(path.to_string_lossy().to_string());
                        }
                    }
                    None
                })
                .collect();
            for file in files {
                self.storage.pdf_files.push(file);
            }
        }

        self
    }

    pub fn build(&mut self, ui: &mut egui::Ui) {
        let current_page = self.current_page.clone();

        let mut route_target: Option<String> = None;

        let app_view = AppView {
            storage: &mut self.storage,
            route: &mut |page| {
                route_target = Some(page.to_string());
            },
        };

        self.pages.get_mut(&current_page).unwrap()
            .build(ui, app_view);

        if let Some(target) = route_target {
            self.route_to(&target);
        }
    }
}

struct AppView<'a> {
    pub storage: &'a mut AppStorage,
    pub route: &'a mut dyn FnMut(&str),
}

trait Page {
    fn build(&mut self, ui: &mut egui::Ui, view: AppView);
}