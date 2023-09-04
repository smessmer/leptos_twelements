use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=package-lock.json");

    // Run "npm install" so twelements are installed and present in `node_modules`.
    let output = Command::new("npm")
        .arg("install")
        .output()
        .expect("Failed to run npm executable");
    if !output.status.success() {
        panic!(
            "npm install failed:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}
