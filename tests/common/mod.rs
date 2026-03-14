use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Helper: run mngit command in a temp directory
pub fn mngit(dir: &Path, args: &[&str]) -> (String, String, bool) {
    let output = Command::new(env!("CARGO_BIN_EXE_mngit"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run mngit");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

/// Create a fresh temp dir for each test
pub fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mngit_test_{name}_{}", std::process::id()));
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir_all(&dir).unwrap();
    dir
}
