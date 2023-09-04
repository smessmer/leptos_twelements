use std::fs;
use std::path::Path;

const PLUGIN_CJS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/node_modules/tw-elements/dist/plugin.cjs"
);
const JS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/node_modules/tw-elements/dist/js"
);
const RS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src");

/// Call this from your `build.rs` to install the Tailwind Elements files to the given target directory.
/// This is necessary so that they can be referenced from your `tailwind.config.js`.
pub fn install_files_to(target_dir: &Path) {
    let target_dir = target_dir.join(".leptos-twelements");
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).expect("Failed to clear target_dir");
    }
    fs::create_dir_all(&target_dir).expect("Failed to create target_dir");

    fs::copy(PLUGIN_CJS_PATH, target_dir.join("plugin.cjs"))
        .expect("Failed to copy twelements plugin.cjs");

    recursively_copy_files(Path::new(JS_PATH), &target_dir.join("js"))
        .expect("Failed to copy twelements js files");
    recursively_copy_files(Path::new(RS_PATH), &target_dir.join("rs"))
        .expect("Failed to copy twelements rs files");
}

fn recursively_copy_files(from: &Path, to: &Path) -> Result<(), std::io::Error> {
    fs::create_dir(&to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            recursively_copy_files(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), &to.join(entry.file_name()))?;
        }
    }
    Ok(())
}
