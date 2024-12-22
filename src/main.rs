mod pages;

use eframe::egui;

struct MyApp {
    context: pages::AppContext,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            context: pages::AppContext::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.push_font(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.context
                .handle_input(ctx)
                .build(ui);
        });
    }
}

impl MyApp {
    fn push_font(&mut self, ctx: &egui::Context) {
        let mut fonts = eframe::egui::FontDefinitions::default();

        let font_data = std::fs::read("resources/NotoSerifSC.ttf")
            .unwrap_or_else(|_| panic!("Failed to read font file"));

        fonts.font_data.insert(
            "NotoSerifSC".to_owned(),
            std::sync::Arc::new(eframe::egui::FontData::from_owned(font_data)),
        );

        fonts.families.get_mut(&eframe::egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "NotoSerifSC".to_owned());

        fonts.families.get_mut(&eframe::egui::FontFamily::Monospace)
            .unwrap()
            .push("NotoSerifSC".to_owned());

        ctx.set_fonts(fonts);
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Watermarker",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}
