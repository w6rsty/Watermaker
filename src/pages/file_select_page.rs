use crate::pages::{AppView, Page};
use rfd::FileDialog;
use eframe::egui;

pub struct FileSelectPage;
impl Page for FileSelectPage {
    fn build(&mut self, ui: &mut egui::Ui, view: AppView) {
        if ui.button("<").clicked() {
            view.storage.pdf_files.clear();
            (view.route)("HomePage");
        }

        self.build_file_select_button(ui, &mut view.storage.pdf_files);
        self.build_file_list(ui, &mut view.storage.pdf_files);


        if !view.storage.pdf_files.is_empty() {
            if ui.button("下一步").on_hover_text("下一步").clicked() {
                (view.route)("EditPage");
            }
        }
    }
}

impl FileSelectPage {
    fn build_file_select_button(&mut self, ui: &mut egui::Ui, pdf_files: &mut Vec<String>) {
        if ui.button("＋ 添加文件").clicked() {
            if let Some(files) = FileDialog::new()
                .add_filter("PDF Files", &["pdf"])
                .pick_files() {
                for file in files {
                    pdf_files.push(file.to_string_lossy().to_string());
                }
            }
        }
    }

    fn build_file_list(&mut self, ui: &mut egui::Ui, pdf_files: &mut Vec<String>) {
        if pdf_files.is_empty() {
            ui.label("尚未选择任何 PDF 文件。");
        } else {
            let mut indices_to_remove = Vec::new();
            for (index, file_path) in pdf_files.iter().enumerate() {
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 20.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.columns(2, |columns| {
                            columns[0].with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                                let file_name = file_path.split('/').last().unwrap_or(file_path);
                                ui.label(file_name).on_hover_text(file_path);
                            });

                            columns[1].with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                if ui.button("❌").clicked() {
                                    indices_to_remove.push(index);
                                }
                            });
                        });
                    },
                );
                ui.separator();
            }
            for index in indices_to_remove.iter().rev() {
                pdf_files.remove(*index);
            }
        }
    }
}