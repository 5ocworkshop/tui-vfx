// <FILE>crates/tui-vfx-compost/tests/test_no_legacy_imports.rs</FILE> - <DESC>Primitive/source port legacy dependency guard</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 legacy policy forbids new primitive/source ports from importing legacy compositor/style/content/shadow crates while existing compost uses are retired separately.</WCTX>
// <CLOG>0.1.0: INIT — scan the new primitive substrate directory for forbidden legacy crate imports.</CLOG>

use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN_IMPORTS: &[&str] = &[
    "tui_vfx_compositor",
    "tui-vfx-compositor",
    "tui_vfx_style",
    "tui-vfx-style",
    "tui_vfx_content",
    "tui-vfx-content",
    "tui_vfx_shadow",
    "tui-vfx-shadow",
];

#[test]
fn primitive_substrate_has_no_legacy_imports() {
    let primitive_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/primitive");
    let files = rust_files_under(&primitive_dir);
    assert!(
        !files.is_empty(),
        "primitive substrate directory is present"
    );

    let mut violations = Vec::new();
    for file in files {
        let text = fs::read_to_string(&file).expect("read primitive source");
        for forbidden in FORBIDDEN_IMPORTS {
            if text.contains(forbidden) {
                violations.push(format!("{} contains {forbidden}", file.display()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "new primitive/source ports must not import legacy crates:\n{}",
        violations.join("\n")
    );
}

fn rust_files_under(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files(root, &mut files);
    files.sort();
    files
}

fn collect_rust_files(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read source directory") {
        let path = entry.expect("read directory entry").path();
        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

// <FILE>crates/tui-vfx-compost/tests/test_no_legacy_imports.rs</FILE> - <DESC>Primitive/source port legacy dependency guard</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
