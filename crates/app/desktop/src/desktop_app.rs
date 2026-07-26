/// 白紙の画面を構成するアプリケーション本体。
pub struct DesktopApp;

impl eframe::App for DesktopApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default_margins().show(ui, |_ui| {});
    }
}
