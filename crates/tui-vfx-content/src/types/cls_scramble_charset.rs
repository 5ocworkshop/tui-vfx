// <FILE>tui-vfx-content/src/types/cls_scramble_charset.rs</FILE> - <DESC>Character sets for Matrix effects</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>OFPF compliance: Rename type files to cls_ prefix</WCTX>
// <CLOG>Renamed from scramble_charset.rs to cls_scramble_charset.rs for OFPF compliance</CLOG>

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ScrambleCharset {
    Alphanumeric,
    Binary,
    Matrix,
    Katakana,
}
impl ScrambleCharset {
    pub fn get_chars(&self) -> &'static [char] {
        match self {
            Self::Alphanumeric => &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5',
                '6', '7', '8', '9',
            ],
            Self::Binary => &['0', '1'],
            Self::Matrix => &[
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ],
            // Simplified Katakana set for demonstration
            Self::Katakana => &[
                '¾', '¿', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í',
            ],
        }
    }
}

// <FILE>tui-vfx-content/src/types/cls_scramble_charset.rs</FILE> - <DESC>Character sets for Matrix effects</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
