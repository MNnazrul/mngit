mod common;

use std::fs;
use common::{mngit, temp_dir};

#[test]
fn test_roundtrip_hash_then_cat() {
    let dir = temp_dir("roundtrip");
    mngit(&dir, &["init"]);

    let content = "round trip test content\nwith newlines\n";
    fs::write(dir.join("rt.txt"), content).unwrap();

    let (stdout, _, _) = mngit(&dir, &["-w", "hash-object", "rt.txt"]);
    let oid = stdout.trim();

    let (cat_out, _, ok) = mngit(&dir, &["cat-file", "-p", oid]);
    assert!(ok);
    assert_eq!(cat_out, content);

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_roundtrip_write_tree_ls_tree_cat_file() {
    let dir = temp_dir("full_roundtrip");
    mngit(&dir, &["init"]);

    let file_content = "fn main() { println!(\"hi\"); }\n";
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/main.rs"), file_content).unwrap();

    let (tree_oid, _, _) = mngit(&dir, &["write-tree"]);
    let tree_oid = tree_oid.trim();

    let (ls_out, _, _) = mngit(&dir, &["ls-tree", tree_oid]);
    let line = ls_out.lines().next().unwrap();
    assert!(line.contains("tree") && line.contains("src"));

    let src_tree_oid = line.split_whitespace().nth(2).unwrap();
    let (ls_sub, _, _) = mngit(&dir, &["ls-tree", src_tree_oid]);
    assert!(ls_sub.contains("main.rs"));

    let blob_oid = ls_sub.lines().next().unwrap().split_whitespace().nth(2).unwrap();
    let (cat_out, _, _) = mngit(&dir, &["cat-file", "-p", blob_oid]);
    assert_eq!(cat_out, file_content);

    fs::remove_dir_all(&dir).unwrap();
}
