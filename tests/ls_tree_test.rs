mod common;

use std::fs;
use common::{mngit, temp_dir};

#[test]
fn test_ls_tree_lists_entries() {
    let dir = temp_dir("ls_tree");
    mngit(&dir, &["init"]);

    fs::write(dir.join("hello.txt"), "hello").unwrap();
    fs::write(dir.join("world.txt"), "world").unwrap();

    let (tree_oid, _, _) = mngit(&dir, &["write-tree"]);
    let tree_oid = tree_oid.trim();

    let (stdout, _, ok) = mngit(&dir, &["ls-tree", tree_oid]);
    assert!(ok);

    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("hello.txt"));
    assert!(lines[1].contains("world.txt"));

    for line in &lines {
        assert!(line.contains("blob"));
        assert!(line.contains("100644"));
    }

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_ls_tree_shows_subdirectory_as_tree() {
    let dir = temp_dir("ls_tree_sub");
    mngit(&dir, &["init"]);

    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/lib.rs"), "// lib").unwrap();
    fs::write(dir.join("README"), "readme").unwrap();

    let (tree_oid, _, _) = mngit(&dir, &["write-tree"]);
    let tree_oid = tree_oid.trim();

    let (stdout, _, ok) = mngit(&dir, &["ls-tree", tree_oid]);
    assert!(ok);

    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2);

    assert!(lines[0].contains("blob") && lines[0].contains("README"));
    assert!(lines[1].contains("tree") && lines[1].contains("src"));

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_ls_tree_fails_on_blob() {
    let dir = temp_dir("ls_tree_blob");
    mngit(&dir, &["init"]);

    fs::write(dir.join("file.txt"), "data").unwrap();
    let (stdout, _, _) = mngit(&dir, &["-w", "hash-object", "file.txt"]);
    let blob_oid = stdout.trim();

    let (_, stderr, ok) = mngit(&dir, &["ls-tree", blob_oid]);
    assert!(!ok);
    assert!(stderr.contains("Not a tree"), "stderr: {stderr}");

    fs::remove_dir_all(&dir).unwrap();
}
