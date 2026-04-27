//! Basic example showing Chinese font loading with egui-chinese-font.
//! 
//! This example demonstrates how to set up Chinese fonts for an egui application
//! and displays various Chinese text samples.

use egui_chinese_support::setup_chinese_fonts;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 600.0])
            .with_title("Chinese Font Demo - 中文字体演示"),
        ..Default::default()
    };

    eframe::run_native(
        "Chinese Font Demo",
        options,
        Box::new(|cc| {
            // Setup Chinese fonts - this is the key line!
            if let Err(e) = setup_chinese_fonts(&cc.egui_ctx) {
                eprintln!("Failed to load Chinese fonts: {}", e);
            }
            
            Ok(Box::new(ChineseFontDemo::default()))
        }),
    )
}

#[derive(Default)]
struct ChineseFontDemo {
    text_input: String,
}

impl eframe::App for ChineseFontDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🇨🇳 Chinese Font Display Test");
            ui.label("egui-chinese-font 演示应用");
            
            ui.separator();
            
            ui.group(|ui| {
                ui.heading("简体中文 (Simplified Chinese)");
                ui.label("你好，世界！欢迎使用 egui-chinese-font。");
                ui.label("这个库可以自动加载系统中的中文字体。");
                ui.label("常用汉字：一二三四五六七八九十百千万");
                ui.label("标点符号：，。；：？！\"\"''（）【】《》");
            });
            
            ui.separator();
            
            ui.group(|ui| {
                ui.heading("繁體中文 (Traditional Chinese)");
                ui.label("你好，世界！歡迎使用 egui-chinese-font。");
                ui.label("這個庫可以自動載入系統中的中文字體。");
                ui.label("常用漢字：壹貳參肆伍陸柒捌玖拾佰仟萬");
                ui.label("標點符號：，。；：？！\"\"''（）【】《》");
            });
            
            ui.separator();
            
            ui.group(|ui| {
                ui.heading("Mixed Text - 混合文本");
                ui.label("English + 中文 + Numbers: 2025年");
                ui.label("Email: user@example.com 电子邮件");
                ui.label("Math: 1 + 1 = 2 数学公式");
                ui.label("Emoji: 🚀 火箭 🌟 星星 ❤️ 爱心");
            });
            
            ui.separator();
            
            ui.group(|ui| {
                ui.heading("Interactive Text - 交互文本");
                ui.label("Type some Chinese text:");
                ui.text_edit_multiline(&mut self.text_input);
                if !self.text_input.is_empty() {
                    ui.label(format!("You typed: {}", self.text_input));
                }
            });
            
            ui.separator();
            
            ui.group(|ui| {
                ui.heading("Font Information - 字体信息");
                ui.label("This application uses egui-chinese-font to automatically");
                ui.label("detect and load Chinese fonts from your system.");
                ui.label("此应用程序使用 egui-chinese-font 自动检测");
                ui.label("并加载系统中的中文字体。");
            });
        });
    }
}
