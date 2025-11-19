mod tray_icon;

use std::process::Command;
use std::error::Error;
use anyhow::Result;

pub const ARGS_APP:&str = "app";

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1{
        let arg1 = args[1].to_lowercase();
        if arg1.starts_with(ARGS_APP) {
            //打开App
            let main = MainWindow::new()?;
            // let handel = main.as_weak();
            main.run()?;
            return Ok(());
        }
    }

    //打开app
    open_app();

    //打开图标
    tray_icon::main().expect("tray_icon.ERROR");

    Ok(())
}

pub fn open_app(){
    let _ = start_process(vec![ARGS_APP.to_string()]);
}

fn start_process(command_args: Vec<String>) -> Result<()>{
    // 获取当前可执行文件的路径
    let current_exe = std::env::current_exe()?;

    // 启动新进程并传递命令行参数
    Command::new(current_exe)
        .args(&command_args)
        .spawn()?;
    Ok(())
}
