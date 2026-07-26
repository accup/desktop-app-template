use crate::menu_bar::menu_bar;

/// 白紙の画面を構成するアプリケーション本体。
pub struct DesktopApp {
    /// フォントライセンスのモーダルを表示中かどうか。
    pub license_modal_open: bool,
}

impl eframe::App for DesktopApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menu").show(ui, |ui| menu_bar(ui, self));
        egui::CentralPanel::default_margins().show(ui, |_ui| {});
        ui::font_license_modal(ui.ctx(), &mut self.license_modal_open);
    }
}
