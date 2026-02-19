// <FILE>tui-vfx-core-macros/src/types.rs</FILE> - <DESC>Internal types for ConfigSchema derive</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted ConfigAttr and ScalarLit</CLOG>

#[derive(Default, Debug, Clone)]
pub(crate) struct ConfigAttr {
    pub(crate) hidden: bool,
    pub(crate) opaque: bool,
    pub(crate) help: Option<String>,
    pub(crate) default: Option<ScalarLit>,
    pub(crate) min: Option<ScalarLit>,
    pub(crate) max: Option<ScalarLit>,
}

#[derive(Debug, Clone)]
pub(crate) enum ScalarLit {
    Bool(bool),
    Number(String),
    String(String),
    Char(char),
}

// <FILE>tui-vfx-core-macros/src/types.rs</FILE> - <DESC>Internal types for ConfigSchema derive</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>

