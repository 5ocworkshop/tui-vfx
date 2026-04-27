// <FILE>tui-vfx-core-macros/src/types.rs</FILE> - <DESC>Internal types for the ConfigSchema derive: ConfigAttr (parsed `#[config(...)]` form), ScalarLit (literal-value form for default/min/max), SerdeAttr (parsed `#[serde(...)]` subset the macro consumes).</DESC>
// <VERS>VERSION: 0.3.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup — move types out of the inline lib.rs body and into this OFPF home alongside the per-function fnc_* / col_* siblings. Adds SerdeAttr (which the abandoned-refactor stub of types.rs lacked).</WCTX>
// <CLOG>0.3.0: MAJOR — replace the abandoned-refactor stub with the live shapes from lib.rs. ConfigAttr and ScalarLit unchanged in shape; SerdeAttr is new (the live macro consumes #[serde(rename, rename_all, skip, default, tag)]). 0.1.1: Extracted ConfigAttr and ScalarLit (stub).</CLOG>

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

#[derive(Default, Debug, Clone)]
pub(crate) struct SerdeAttr {
    pub(crate) rename: Option<String>,     // #[serde(rename = "...")]
    pub(crate) rename_all: Option<String>, // #[serde(rename_all = "snake_case")]
    pub(crate) skip: bool,                 // #[serde(skip)]
    pub(crate) default: bool,              // #[serde(default)]
    pub(crate) tag: Option<String>,        // #[serde(tag = "type")]
}

// <FILE>tui-vfx-core-macros/src/types.rs</FILE> - <DESC>Internal types for the ConfigSchema derive</DESC>
// <VERS>END OF VERSION: 0.3.0 - 2026-04-28</VERS>
