mod tray_icon;

use hide_console::hide_console;
use std::error::Error;
use anyhow::Result;

pub const ARGS_APP:&str = "app";

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    #![windows_subsystem = "windows"]

    // hide_console();

    //打开App
    let main = MainWindow::new().expect("Failed to create main window");
    let handel = main.as_weak();
    // handel.on_close_requested(move || {
    //     println!("Window close requested, hiding instead.");
    //     // 当用户点击关闭时，隐藏窗口
    //     if let Some(window) = handel.upgrade() {
    //         window.hide();
    //     }
    //     true // 返回 true 告诉 Slint 我们处理了这个事件
    // });

    let handel_clone = handel.clone();
    //打开图标
    // tray_icon::main(|| {
        // if let Some(window) = handel_clone.upgrade() {
        //     if window.is_visible() {
        //
        //     } else {
        //         window.show();
        //     }
        //     window.raise(); // 把窗口提到最前面
        // }
    // }).expect("tray_icon.ERROR");

    tray_icon::main(move|| {
        // if let Some(window) = handel_clone.upgrade() {
        //     if window.is_visible() {
        //     } else {
        //         window.show();
        //     }
        //     window.raise(); // 把窗口提到最前面
        // }
    }).expect("tray_icon.ERROR");

    main.show();

    // 启动 Slint 的事件循环
    slint::run_event_loop().expect("Failed to run event loop");

    Ok(())
}
