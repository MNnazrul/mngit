mod common;

use std::fs;
use common::{mngit, temp_dir};

#[test]
fn test_write_tree_returns_oid() {
    let dir = temp_dir("write_tree");
    mngit(&dir, &["init"]);

    fs::write(dir.join("file.txt"), "content").unwrap();

    let (stdout, _, ok) = mngit(&dir, &["write-tree"]);
    assert!(ok);

    let oid = stdout.trim();
    assert_eq!(oid.len(), 40);
    assert!(oid.chars().all(|c| c.is_ascii_hexdigit()));

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_write_tree_deterministic() {
    let dir = temp_dir("write_tree_det");
    mngit(&dir, &["init"]);

    fs::write(dir.join("a.txt"), "aaa").unwrap();
    fs::write(dir.join("b.txt"), "bbb").unwrap();

    let (oid1, _, _) = mngit(&dir, &["write-tree"]);
    let (oid2, _, _) = mngit(&dir, &["write-tree"]);
    assert_eq!(oid1.trim(), oid2.trim(), "write-tree should be deterministic");

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_write_tree_with_subdirectory() {
    let dir = temp_dir("write_tree_sub");
    mngit(&dir, &["init"]);

    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/main.rs"), "fn main() {}").unwrap();
    fs::write(dir.join("README"), "hello").unwrap();

    let (stdout, _, ok) = mngit(&dir, &["write-tree"]);
    assert!(ok);
    assert_eq!(stdout.trim().len(), 40);

    fs::remove_dir_all(&dir).unwrap();
}
