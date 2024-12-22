use crate::pages::{AppView, Page};
use rfd::FileDialog;
use eframe::egui;

pub struct HomePage;
impl Page for HomePage {
    fn build(&mut self, ui: &mut egui::Ui, view: AppView) {
        ui.heading("PDF Watermarker");
        
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
            if ui.button("＋ 选择或拖入文件").clicked() {
                if let Some(files) = FileDialog::new()
                    .add_filter("PDF Files", &["pdf"])
                    .pick_files() {
                    for file in files {
                        view.storage.pdf_files.push(file.to_string_lossy().to_string());
                    }
                }
            }
        });

        if !view.storage.pdf_files.is_empty() {
            (view.route)("FileSelectPage");
        }
    }
}
