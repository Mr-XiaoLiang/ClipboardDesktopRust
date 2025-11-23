mod tray_icon;

use anyhow::Result;
use hide_console::hide_console;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    #![windows_subsystem = "windows"]

    hide_console();

    // 1. 创建一个通道用于与托盘图标线程通信
    let (show_tx, show_rx) = std::sync::mpsc::channel::<()>();

    // 2. 先启动托盘图标线程（在创建窗口之前）
    let tray_thread = std::thread::spawn(move || {
        // 创建一个闭包，用于在托盘图标被点击时发送消息
        let show_window = move || {
            println!("show_window called"); // 调试信息
            // 发送消息到主线程
            show_tx
                .send(())
                .expect("Failed to send show window message");
        };

        // 运行托盘图标
        tray_icon::main(show_window).expect("Failed to initialize tray icon");
    });

    // 3. 创建主窗口
    let main_window = MainWindow::new()?;

    // 4. 启动一个线程来监听托盘图标消息
    std::thread::spawn(move || {
        // 监听来自托盘图标的消息
        while let Ok(()) = show_rx.recv() {
            // 由于Slint窗口通常应该在主线程中操作，
            // 这里我们可以考虑使用Slint的事件循环代理来处理
            // 或者简化处理，让窗口始终可见
        }
    });

    // 5. 运行主窗口（这会启动事件循环）
    main_window.run()?;

    // 等待托盘线程结束
    if let Err(e) = tray_thread.join() {
        eprintln!("托盘线程错误: {:?}", e);
    }

    Ok(())
}
