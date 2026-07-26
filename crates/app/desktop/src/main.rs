//! 白紙のウィンドウを表示するデスクトップアプリケーション。

// Windows でコンソールウィンドウを伴わずにアプリケーションウィンドウのみを起動するため、
// GUI サブシステムを指定する
#![cfg_attr(windows, windows_subsystem = "windows")]

mod desktop_app;
mod menu_bar;

use desktop_app::DesktopApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    eframe::run_native(
        "winker",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(design::font_definitions());
            Ok(Box::new(DesktopApp {
                license_modal_open: false,
            }))
        }),
    )?;
    Ok(())
}
