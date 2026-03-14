mod common;

use std::fs;
use common::{mngit, temp_dir};

#[test]
fn test_hash_object_prints_oid() {
    let dir = temp_dir("hash_obj");
    mngit(&dir, &["init"]);

    fs::write(dir.join("hello.txt"), "hello world\n").unwrap();
    let (stdout, _, ok) = mngit(&dir, &["hash-object", "hello.txt"]);
    assert!(ok);

    let oid = stdout.trim();
    assert_eq!(oid.len(), 40, "OID should be 40 hex chars");
    assert!(oid.chars().all(|c| c.is_ascii_hexdigit()));

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_hash_object_same_content_same_oid() {
    let dir = temp_dir("hash_same");
    mngit(&dir, &["init"]);

    fs::write(dir.join("a.txt"), "same content").unwrap();
    fs::write(dir.join("b.txt"), "same content").unwrap();

    let (oid_a, _, _) = mngit(&dir, &["hash-object", "a.txt"]);
    let (oid_b, _, _) = mngit(&dir, &["hash-object", "b.txt"]);
    assert_eq!(oid_a.trim(), oid_b.trim());

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_hash_object_write_creates_object_file() {
    let dir = temp_dir("hash_write");
    mngit(&dir, &["init"]);

    fs::write(dir.join("hello.txt"), "hello world\n").unwrap();
    let (stdout, _, ok) = mngit(&dir, &["-w", "hash-object", "hello.txt"]);
    assert!(ok);

    let oid = stdout.trim();
    let obj_path = dir
        .join(".mngit/objects")
        .join(&oid[..2])
        .join(&oid[2..]);
    assert!(obj_path.exists(), "object file should exist after -w");

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_hash_object_without_write_no_object_file() {
    let dir = temp_dir("hash_no_write");
    mngit(&dir, &["init"]);

    fs::write(dir.join("test.txt"), "data").unwrap();
    let (stdout, _, _) = mngit(&dir, &["hash-object", "test.txt"]);

    let oid = stdout.trim();
    let obj_path = dir
        .join(".mngit/objects")
        .join(&oid[..2])
        .join(&oid[2..]);
    assert!(!obj_path.exists(), "object file should NOT exist without -w");

    fs::remove_dir_all(&dir).unwrap();
}
