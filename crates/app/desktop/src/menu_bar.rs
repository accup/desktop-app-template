use crate::desktop_app::DesktopApp;

/// フォントライセンスのモーダルを開くボタンを表示する。
pub fn menu_bar(ui: &mut egui::Ui, app: &mut DesktopApp) {
    ui.horizontal(|ui| {
        if ui.button("ライセンス").clicked() {
            app.license_modal_open = true;
        }
    });
}
