use crate::pages::{AppView, Page, AppStorage};
use eframe::egui;

pub struct EditPage;

impl Page for EditPage {
    fn build(&mut self, ui: &mut egui::Ui, view: AppView) {
        if ui.button("<").clicked() {
            (view.route)("FileSelectPage");
        }

        let left_ratio = 0.5;
        let right_ratio = 1.0 - left_ratio;
        let total_width = ui.available_width();

        ui.horizontal(|ui| {
            // 左侧编辑区
            ui.allocate_ui(
                egui::vec2(total_width * left_ratio, ui.available_height()),
                |ui| {
                    ui.vertical(|ui| {
                        self.build_editor(ui, view.storage);
                        
                        if ui.button("下一步").clicked() {
                            (view.route)("OutputPage");
                        }
                    });
                },
            );

            // 右侧预览区
            ui.allocate_ui(
                egui::vec2(total_width * right_ratio, ui.available_height()),
                |ui| {
                    self.build_preview(ui, view.storage);
                }
            );
        });
    }
}

impl EditPage {
    fn build_editor(&mut self, ui: &mut egui::Ui, storage: &mut AppStorage) {
        ui.vertical(|ui| {
            // 文字内容
            ui.text_edit_singleline(&mut storage.text);
            // 颜色
            ui.color_edit_button_srgba(&mut storage.color);
            // X/Y 归一化坐标
            ui.add(egui::Slider::new(&mut storage.position.x, 0.0..=1.0).text("X"));
            ui.add(egui::Slider::new(&mut storage.position.y, 0.0..=1.0).text("Y"));
            // 旋转角度（度数）
            ui.add(egui::Slider::new(&mut storage.rotation, -180.0..=180.0).text("Rotation"));
            // 字体大小
            ui.add(egui::Slider::new(&mut storage.size, 0.1..=100.0).text("Size"));
        });       
    }

    fn build_preview(&mut self, ui: &mut egui::Ui, storage: &mut AppStorage) {
        // -------------------------
        // 1. 计算 A4 预览区域大小
        // -------------------------
        let available_width = ui.available_width();
        let a4_aspect_ratio = 210.0 / 297.0; // A4 比例 (宽 / 高)

        // 这里让矩形宽度占可用空间的 80%，可自行调节
        let rect_width = available_width * 0.8;
        let rect_height = rect_width / a4_aspect_ratio;
        
        // 将 A4 预览矩形放到当前 UI 区域的中心
        let rect = egui::Rect::from_min_size(
            ui.max_rect().min,
            egui::vec2(rect_width, rect_height)
        );

        // 获取画笔并绘制 A4 白纸的轮廓
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, egui::Color32::WHITE);
        painter.rect_stroke(rect, 0.0, (1.0, egui::Color32::GRAY));

        // -------------------------
        // 2. 计算水印文本的位置
        // -------------------------
        // 将 (0,0)~(1,1) 的归一化坐标映射到 A4 预览矩形上
        let text_x = egui::lerp(rect.x_range(), storage.position.x);
        let text_y = egui::lerp(rect.y_range(), storage.position.y);
        let text_pos = egui::pos2(text_x, text_y);

        // -------------------------
        // 3. 绘制水印文本（带旋转）
        // -------------------------
        // 字体大小 & 家族
        let font_id = egui::FontId {
            size: storage.size,
            family: egui::FontFamily::Proportional,
        };

        // 先将输入的度数转为弧度
        let angle_radians = storage.rotation.to_radians();

        // 排版对象
        let galley = ui.fonts(|fonts| {
            fonts.layout_no_wrap(
                storage.text.clone(),
                font_id,
                storage.color
            )
        });

        // 构造要绘制的 TextShape
        let text_shape = egui::epaint::TextShape {
            pos: text_pos, // 左上角坐标
            galley,
            override_text_color: Some(storage.color),
            underline: egui::Stroke::default(),
            angle: angle_radians,
            fallback_color: egui::Color32::TRANSPARENT,
            opacity_factor: 1.0,
        };

        // 最后将其加入 painter
        painter.add(egui::Shape::Text(text_shape));
    }
}