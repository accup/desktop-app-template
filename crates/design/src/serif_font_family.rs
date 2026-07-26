/// セリフ体で描画するためのフォントファミリーを返す。
///
/// 返すファミリーは `font_definitions` の返り値に登録されており、
/// そのフォント定義を適用したコンテキストでのみ使用できる。
#[must_use]
pub fn serif_font_family() -> egui::FontFamily {
    egui::FontFamily::Name("serif".into())
}
