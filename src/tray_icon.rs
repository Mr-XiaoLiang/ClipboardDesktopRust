use std::time::Duration;

use tao::event_loop::{EventLoopBuilder, ControlFlow};
use tray_icon::{menu::{Menu, MenuItem, MenuEvent}, TrayIconBuilder, Icon, TrayIconEvent, ClickType};
use anyhow::Result;
use image;

const ICON: &[u8] = include_bytes!("./icon/tray_icon.png");

// pub fn icon_only(show_window: fn() -> ()) -> Result<()> {
//     let icon = load_icon()?;
//     let show_item = MenuItem::new("打开", true, None);
//     let quit_item = PredefinedMenuItem::quit(None);
//
//     let mut menu = Menu::new();
//     menu.append(&show_item).unwrap();
//     menu.append(&quit_item).unwrap();
//
//     let show_item_clone = show_item.clone();
//     let _tray_icon = Some(
//         TrayIconBuilder::new()
//             .with_menu(Box::new(menu))
//             .with_tooltip("Clipboard")
//             .with_icon(icon)
//             .build()?,
//     );
//     Ok(())
// }

pub fn main<F>(show_window: F) -> Result<()>
where 
    F: Fn() -> () + 'static,
{
    let icon = load_icon()?;

    let event_loop = EventLoopBuilder::new().build();

    let menu = Menu::new();
    menu.append(&MenuItem::new("打开", true, None))?;
    menu.append(&MenuItem::new("退出", true, None))?;

    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Clipboard")
            .with_icon(icon)
            .build()?,
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let event_loop_proxy = event_loop.create_proxy();
    std::thread::spawn(move || {
        loop {
            event_loop_proxy.send_event(()).ok();
            std::thread::sleep(Duration::from_millis(50));
        }
    });

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(MenuEvent { id }) = menu_channel.try_recv() {
            if id.0 == "1001"{
                //点击“打开”菜单，启动主窗口进程
                show_window();
            }else{
                //退出托盘程序
                *control_flow = ControlFlow::Exit;
            }
        }

        if let Ok(TrayIconEvent {click_type, id: _, x: _, y: _, icon_rect: _ }) = tray_channel.try_recv(){
            if let ClickType::Left = click_type{
                //打开主窗口进程
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
    Ok(tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
}
