//! 白紙のウィンドウを表示するデスクトップアプリケーション。

// Windows でコンソールウィンドウを伴わずにアプリケーションウィンドウのみを起動するため、
// GUI サブシステムを指定する
#![cfg_attr(windows, windows_subsystem = "windows")]

mod desktop_app;

use desktop_app::DesktopApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "winker",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(DesktopApp))),
    )?;
    Ok(())
}
