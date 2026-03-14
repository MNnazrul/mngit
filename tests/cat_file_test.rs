mod common;

use std::fs;
use common::{mngit, temp_dir};

#[test]
fn test_cat_file_reads_blob() {
    let dir = temp_dir("cat_file");
    mngit(&dir, &["init"]);

    let content = "hello mngit!\n";
    fs::write(dir.join("greet.txt"), content).unwrap();

    let (stdout, _, _) = mngit(&dir, &["-w", "hash-object", "greet.txt"]);
    let oid = stdout.trim();

    let (cat_out, _, ok) = mngit(&dir, &["cat-file", "-p", oid]);
    assert!(ok);
    assert_eq!(cat_out, content);

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_cat_file_missing_object() {
    let dir = temp_dir("cat_missing");
    mngit(&dir, &["init"]);

    let fake_oid = "aa".to_string() + &"bb".repeat(19);
    let (_, stderr, ok) = mngit(&dir, &["cat-file", "-p", &fake_oid]);
    assert!(!ok);
    assert!(
        stderr.contains("failed") || stderr.contains("not found") || stderr.contains("Not found"),
        "stderr: {stderr}"
    );

    fs::remove_dir_all(&dir).unwrap();
}
