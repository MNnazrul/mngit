mod common;

use std::fs;
use common::{mngit, temp_dir};

#[test]
fn test_init_creates_mngit_dir() {
    let dir = temp_dir("init");
    let (_, _, ok) = mngit(&dir, &["init"]);
    assert!(ok);
    assert!(dir.join(".mngit").is_dir());
    assert!(dir.join(".mngit/objects").is_dir());
    assert!(dir.join(".mngit/refs/heads").is_dir());
    assert!(dir.join(".mngit/HEAD").exists());

    let head = fs::read_to_string(dir.join(".mngit/HEAD")).unwrap();
    assert_eq!(head, "ref: refs/heads/main\n");

    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn test_init_is_idempotent() {
    let dir = temp_dir("init_idem");
    mngit(&dir, &["init"]);
    let (_, _, ok) = mngit(&dir, &["init"]);
    assert!(ok);

    let head = fs::read_to_string(dir.join(".mngit/HEAD")).unwrap();
    assert_eq!(head, "ref: refs/heads/main\n");

    fs::remove_dir_all(&dir).unwrap();
}
