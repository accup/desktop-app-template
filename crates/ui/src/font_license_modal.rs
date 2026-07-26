use design::{FONT_LICENSES, LICENSE_MODAL_WIDTH};

/// 同梱フォントのライセンス表記を確認するモーダルを表示する。
///
/// `open` が真の間だけ表示し、閉じる操作を受け付けた時点で `open` を偽へ戻す。
pub fn font_license_modal(ctx: &egui::Context, open: &mut bool) {
    if !*open {
        return;
    }
    let response = egui::Modal::new(egui::Id::new("font_license_modal")).show(ctx, |ui| {
        ui.set_width(LICENSE_MODAL_WIDTH);
        ui.heading("フォントライセンス");
        egui::ScrollArea::vertical()
            .max_height(0.7 * ctx.content_rect().height())
            .show(ui, |ui| {
                for license in &FONT_LICENSES {
                    ui.collapsing(license.name, |ui| {
                        ui.label(license.text);
                    });
                }
            });
        ui.button("閉じる").clicked()
    });
    if response.inner || response.should_close() {
        *open = false;
    }
}
