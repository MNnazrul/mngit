mod common;

use std::fs;
use common::{mngit, temp_dir};

#[test]
fn test_unknown_command() {
    let dir = temp_dir("unknown_cmd");
    let (_, stderr, ok) = mngit(&dir, &["foobar"]);
    assert!(!ok);
    assert!(stderr.contains("No a correct command"));

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_commands_fail_without_init() {
    let dir = temp_dir("no_init");
    fs::write(dir.join("f.txt"), "x").unwrap();

    let (_, stderr, ok) = mngit(&dir, &["-w", "hash-object", "f.txt"]);
    assert!(!ok);
    assert!(stderr.contains("init"));

    let (_, stderr, ok) = mngit(&dir, &["write-tree"]);
    assert!(!ok);
    assert!(stderr.contains("init"));

    let (_, stderr, ok) = mngit(&dir, &["cat-file", "-p", "abcd1234abcd1234abcd1234abcd1234abcd1234"]);
    assert!(!ok);
    assert!(stderr.contains("init"));

    let (_, stderr, ok) = mngit(&dir, &["ls-tree", "abcd1234abcd1234abcd1234abcd1234abcd1234"]);
    assert!(!ok);
    assert!(stderr.contains("init"));

    fs::remove_dir_all(&dir).unwrap();
}
