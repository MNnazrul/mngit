use std::io;

use crate::objects::{self, commit};

pub fn run(tree_oid: &str, message: &str, parent_oid: Option<&str>) -> io::Result<String> {
    // Verify the tree object exists
    let raw = objects::read_object(tree_oid)?;
    let header_end = raw
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid object format"))?;
    let header = std::str::from_utf8(&raw[..header_end])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    if !header.starts_with("tree ") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Not a tree object: {}", header),
        ));
    }

    let commit_raw = commit::build_commit_raw(tree_oid, parent_oid, message);
    objects::hash_and_store(&commit_raw)
}
