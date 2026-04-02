use std::time::SystemTime;

pub fn build_commit_raw(tree_oid: &str, parent_oid: Option<&str>, message: &str) -> Vec<u8> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let author = format!("mngit <mngit@local> {} +0000", timestamp);

    let mut content = String::new();
    content.push_str(&format!("tree {}\n", tree_oid));
    if let Some(parent) = parent_oid {
        content.push_str(&format!("parent {}\n", parent));
    }
    content.push_str(&format!("author {}\n", author));
    content.push_str(&format!("committer {}\n", author));
    content.push_str(&format!("\n{}\n", message));

    let mut raw = Vec::new();
    raw.extend_from_slice(format!("commit {}\0", content.len()).as_bytes());
    raw.extend_from_slice(content.as_bytes());
    raw
}
