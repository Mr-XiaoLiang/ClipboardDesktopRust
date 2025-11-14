
use eframe::egui;
use crate::ClipboardDesktopApp;

pub fn app_content(app: &mut ClipboardDesktopApp, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut egui::Ui) {

    ui.heading("Hello World!");

    ui.button("Click me");

}
