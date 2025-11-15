mod content;

use content::*;
use eframe::egui;
use std::sync::Arc;

fn main() {
    // 创建默认的原生窗口配置
    let mut native_options = eframe::NativeOptions::default();
    native_options.centered = true;
    // 运行原生窗口
    eframe::run_native(
        // 设置窗口标题
        "ClipboardDesktop",
        // 配置原生窗口参数
        native_options,
        // 创建App实例
        Box::new(|cc| Ok(Box::new(ClipboardDesktopApp::new(cc)))),
    )
    .expect("ClipboardDesktopApp.ERROR");
}

// 属性宏，自动实现 Default trait
#[derive(Default)]
// 定义空结构体，类似 Java 的 class
pub struct ClipboardDesktopApp {
    items: Vec<String>,
}

// 为结构体实现方法，类似 Java 的 class 内部方法
impl ClipboardDesktopApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 定义关联函数 new（构造函数替代），返回 Self（自身类型）
        // cc 参数是 CreationContext 的引用，'_ 表示生命周期参数
        // 类比 Java：static ClipboardDesktopApp new(CreationContext cc)

        // 调用默认实现创建实例
        // 类比 Java：return new ClipboardDesktopApp();
        Self::default()
    }

    fn load_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            Arc::from(egui::FontData::from_static(include_bytes!("xxxxx.ttf"))),
        );
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("my_font".to_owned());
        ctx.set_fonts(fonts);
    }
}

// 实现 App trait（类似 Java 接口）
// 类比 Java：class ClipboardDesktopApp implements App
impl eframe::App for ClipboardDesktopApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 实现 update 方法，&mut self 相当于 Java 的 this，可变借用
        // ctx 和 frame 是引用参数
        // 类比 Java：void update(Context ctx, Frame frame)

        egui::CentralPanel::default().show(ctx, |ui| {
            // 调用 CentralPanel 的 show 方法，第二个参数是闭包
            // 类比 Java：centralPanel.show(ctx, (ui) -> { ... })
            app_content(self, ctx, frame, ui);
        });
    }
}
