use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 380.0])
            .with_icon(demo::create_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "生肖计算器",
        options,
        Box::new(|_cc| {
            #[cfg(not(target_os = "android"))]
            demo::setup_fonts(_cc);
            Ok(Box::new(demo::ZodiacApp::default()))
        }),
    )
}
