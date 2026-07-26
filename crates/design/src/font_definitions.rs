use std::sync::Arc;

use crate::serif_font_family::serif_font_family;

/// 同梱フォントを登録したフォント定義を返す。
///
/// サンセリフ体（既定の `Proportional` ファミリー）には Roboto と LINE Seed JP を
/// この優先順で割り当て、セリフ体（`serif_font_family` のファミリー）には
/// しっぽり明朝を割り当てる。既定のフォントは絵文字などの代替として残す。
#[must_use]
pub fn font_definitions() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "roboto".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../../fonts/Roboto/static/Roboto-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "line_seed_jp".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../../fonts/LINE_Seed_JP/LINESeedJP-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "shippori_mincho".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../../fonts/Shippori_Mincho/ShipporiMincho-Regular.ttf"
        ))),
    );
    let proportional = fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    proportional.insert(0, "roboto".to_owned());
    proportional.insert(1, "line_seed_jp".to_owned());
    let serif = std::iter::once("shippori_mincho".to_owned())
        .chain(proportional.iter().cloned())
        .collect();
    fonts.families.insert(serif_font_family(), serif);
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("line_seed_jp".to_owned());
    fonts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 既定のファミリーでサンセリフ体を優先する() {
        let fonts = font_definitions();
        let proportional = &fonts.families[&egui::FontFamily::Proportional];
        assert_eq!(proportional[0], "roboto");
        assert_eq!(proportional[1], "line_seed_jp");
    }

    #[test]
    fn セリフ体のファミリーでしっぽり明朝を優先する() {
        let fonts = font_definitions();
        let serif = &fonts.families[&serif_font_family()];
        assert_eq!(serif[0], "shippori_mincho");
    }

    #[test]
    fn 等幅ファミリーの代替に日本語フォントを含む() {
        let fonts = font_definitions();
        let monospace = &fonts.families[&egui::FontFamily::Monospace];
        assert!(monospace.contains(&"line_seed_jp".to_owned()));
    }

    #[test]
    fn 登録するフォント名がフォントデータの定義と対応する() {
        let fonts = font_definitions();
        for names in fonts.families.values() {
            for name in names {
                assert!(fonts.font_data.contains_key(name), "{name} が未定義");
            }
        }
    }
}
