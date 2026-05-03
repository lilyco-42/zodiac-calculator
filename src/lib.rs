use eframe::egui;

const ZODIAC: [char; 12] = [
    '鼠', '牛', '虎', '兔', '龙', '蛇', '马', '羊', '猴', '鸡', '狗', '猪',
];

const ZODIAC_EMOJI: [&str; 12] = [
    "🐭", "🐮", "🐯", "🐰", "🐲", "🐍", "🐴", "🐏", "🐵", "🐔", "🐶", "🐷",
];

pub struct ZodiacApp {
    year_input: String,
    zodiac_index: Option<usize>,
    error: Option<String>,
}

impl Default for ZodiacApp {
    fn default() -> Self {
        Self {
            year_input: String::new(),
            zodiac_index: None,
            error: None,
        }
    }
}

impl eframe::App for ZodiacApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(24.0);

                ui.add(egui::Label::new(
                    egui::RichText::new("生肖计算器")
                        .size(28.0)
                        .color(egui::Color32::from_rgb(201, 168, 76)),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new("CHINESE ZODIAC CALCULATOR")
                        .size(10.0)
                        .color(egui::Color32::from_rgb(139, 115, 85))
                        .weak(),
                ));

                ui.add_space(8.0);
                let divider_width = 200.0;
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(divider_width, 1.0), egui::Sense::hover());
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.left(), rect.center().y),
                        egui::pos2(rect.right(), rect.center().y),
                    ],
                    egui::Stroke::new(
                        1.0,
                        egui::Color32::from_rgba_premultiplied(201, 168, 76, 60),
                    ),
                );

                ui.add_space(32.0);

                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(egui::RichText::new("出生年份").size(14.0)));
                    ui.add(egui::TextEdit::singleline(&mut self.year_input).desired_width(100.0));
                    ui.add(egui::Label::new(egui::RichText::new("年").size(14.0)));
                });

                ui.add_space(16.0);

                if ui
                    .add_sized(
                        egui::vec2(160.0, 36.0),
                        egui::Button::new(egui::RichText::new("计算").size(16.0))
                            .fill(egui::Color32::from_rgb(194, 59, 34))
                            .stroke(egui::Stroke::NONE),
                    )
                    .clicked()
                {
                    match self.year_input.trim().parse::<usize>() {
                        Ok(year) => {
                            self.zodiac_index = Some((year + 8) % 12);
                            self.error = None;
                        }
                        Err(_) => {
                            self.zodiac_index = None;
                            self.error = Some("请输入合法的年份数字".to_string());
                        }
                    }
                }

                ui.add_space(36.0);

                if let Some(idx) = self.zodiac_index {
                    ui.label(egui::RichText::new(ZODIAC_EMOJI[idx]).size(72.0));
                    ui.add_space(6.0);
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("你的生肖是：{}", ZODIAC[idx]))
                            .size(24.0)
                            .color(egui::Color32::from_rgb(245, 230, 200)),
                    ));
                } else if let Some(ref err) = self.error {
                    ui.add_space(8.0);
                    ui.add(egui::Label::new(
                        egui::RichText::new(err)
                            .size(15.0)
                            .color(egui::Color32::from_rgb(194, 59, 34)),
                    ));
                }
            });
        });
    }
}

pub fn create_icon() -> egui::IconData {
    let size = 64;
    let mut rgba = Vec::with_capacity(size * size * 4);
    let center = size as f32 / 2.0;
    let radius = center - 1.5;
    let inner_radius = radius * 0.55;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let dist = (dx * dx + dy * dy).sqrt();

            let (r, g, b, a) = if dist > radius {
                (0, 0, 0, 0)
            } else if dist > radius - 2.0 {
                (201, 168, 76, 255)
            } else if dist > inner_radius {
                (26, 26, 46, 255)
            } else {
                (22, 33, 62, 255)
            };
            rgba.extend_from_slice(&[r, g, b, a]);
        }
    }

    egui::IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}

pub fn setup_fonts(cc: &eframe::CreationContext) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "chinese".to_owned(),
        egui::FontData::from_static(include_bytes!("../msyh-subset.ttf")).into(),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "chinese".to_owned());
    cc.egui_ctx.set_fonts(fonts);
}

// --- Android entry point (compiled only for Android targets) ---
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(_app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 380.0])
            .with_app_id("com.zodiac.calculator"),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "生肖计算器",
        options,
        Box::new(|cc| {
            setup_fonts(cc);
            Ok(Box::new(ZodiacApp::default()))
        }),
    );
}
