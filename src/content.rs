
use eframe::egui;
use crate::ClipboardDesktopApp;

pub fn app_content(app: &mut ClipboardDesktopApp, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {
    // 创建一排按钮
    ui.horizontal(|ui|{
        if ui.button("Dark").clicked(){
            ctx.set_visuals(egui::Visuals::dark());
        }
        if ui.button("Light").clicked(){
            ctx.set_visuals(egui::Visuals::light());
        }
    });

    // 读取列表然后显示
    for item in &app.items {
        ui.heading(item);
    }

    ui.heading("Hello World!");
    ui.heading("中文内容");

}
