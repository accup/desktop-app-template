use crate::font_license::FontLicense;

/// 同梱するすべてのフォントのライセンス表記。
pub const FONT_LICENSES: [FontLicense; 3] = [
    FontLicense {
        name: "Roboto",
        text: include_str!("../../../fonts/Roboto/OFL.txt"),
    },
    FontLicense {
        name: "LINE Seed JP",
        text: include_str!("../../../fonts/LINE_Seed_JP/OFL.txt"),
    },
    FontLicense {
        name: "しっぽり明朝",
        text: include_str!("../../../fonts/Shippori_Mincho/OFL.txt"),
    },
];
