use std::time::Duration;

use tao::event_loop::{EventLoopBuilder, ControlFlow};
// 添加Windows平台特定的扩展导入
#[cfg(windows)]
use tao::platform::windows::EventLoopBuilderExtWindows;
use tray_icon::{menu::{Menu, MenuItem, MenuEvent}, TrayIconBuilder, Icon, TrayIconEvent, ClickType};
use anyhow::Result;
use image;

const ICON: &[u8] = include_bytes!("./icon/tray_icon.png");

pub fn main<F>(show_window: F) -> Result<()>
where
    F: Fn() -> () + 'static,
{
    let icon = load_icon()?;

    // 创建事件循环构建器并配置
    let event_loop = {
        #[cfg(windows)]
        {
            // 在Windows上配置允许在任何线程创建事件循环
            EventLoopBuilder::new().with_any_thread(true).build()
        }
        #[cfg(not(windows))]
        {
            // 在非Windows平台上直接构建
            EventLoopBuilder::new().build()
        }
    };

    // 创建菜单并设置ID
    let menu = Menu::new();
    let open_item = MenuItem::new("打开", true, None);
    let exit_item = MenuItem::new("退出", true, None);
    menu.append(&open_item)?;
    menu.append(&exit_item)?;

    // 创建托盘图标
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Clipboard")
        .with_icon(icon)
        .build()?;

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || {
        loop {
            // 发送事件以保持事件循环活跃
            event_loop_proxy.send_event(()).ok();
            std::thread::sleep(Duration::from_millis(50));
        }
    });

    // 移动菜单项到事件循环闭包中，确保它们的生命周期足够长
    event_loop.run(move |_event, _, control_flow| {
        // 移动菜单项到闭包中，避免生命周期问题
        let _keep_alive = (&open_item, &exit_item);

        // 获取菜单项ID
        let open_id = open_item.id();
        let exit_id = exit_item.id();

        *control_flow = ControlFlow::Wait;

        // 处理菜单点击事件
        if let Ok(MenuEvent { id }) = menu_channel.try_recv() {
            println!("Menu item clicked: {:?}", id); // 调试信息
            if id == open_id {
                // 点击"打开"菜单
                show_window();
            } else if id == exit_id {
                // 退出程序
                *control_flow = ControlFlow::Exit;
            }
        }

        // 处理托盘图标点击事件
        if let Ok(TrayIconEvent {click_type, .. }) = tray_channel.try_recv() {
            println!("Tray icon clicked: {:?}", click_type); // 调试信息
            if let ClickType::Left = click_type {
                // 左键点击打开主窗口
                show_window();
            }
        }
    });
}

fn load_icon() -> Result<Icon>{
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory_with_format(ICON, image::ImageFormat::Png)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    println!("Icon loaded: {}x{}", icon_width, icon_height); // 调试信息
    Ok(tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
}