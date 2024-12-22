use crate::pages::{AppView, Page, AppStorage};
use eframe::egui::{self, vec2};
use rfd::FileDialog;
use lopdf::{
    Document, Object, Stream,
    content::{Content, Operation},
    dictionary, Error as LopdfError
};

pub struct OutputPage;

impl Page for OutputPage {
    fn build(&mut self, ui: &mut egui::Ui, view: AppView) {
        ui.vertical(|ui| {
            if ui.button("<").clicked() {
                (view.route)("EditPage");
            }   
            ui.label("保存到");
            
            let but = egui::Button::new(view.storage.target_dir.clone())
                .min_size(vec2(ui.available_width() * 0.8, 20.0));

            if ui.add(but).clicked() {
                if let Some(dir) = FileDialog::new()
                    .pick_folder() {
                    view.storage.target_dir = dir.to_string_lossy().to_string();
                }
            }

            if !view.storage.target_dir.is_empty() {
                if ui.button("生成").clicked() {
                    
                }
            }
        });
    }
}