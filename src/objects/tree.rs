use std::io;

#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub oid: Vec<u8>, // 20 raw bytes
}

pub fn build_tree_raw(entries: &[TreeEntry]) -> Vec<u8> {
    let mut content = Vec::new();

    for entry in entries {
        // Format: "<mode> <name>\0<20-byte-oid>"
        content.extend_from_slice(entry.mode.as_bytes());
        content.push(b' ');
        content.extend_from_slice(entry.name.as_bytes());
        content.push(0);
        content.extend_from_slice(&entry.oid);
    }

    let mut raw = Vec::with_capacity(32 + content.len());
    raw.extend_from_slice(format!("tree {}\0", content.len()).as_bytes());
    raw.extend_from_slice(&content);
    raw
}

pub fn parse_tree_entries(raw: &[u8]) -> io::Result<Vec<TreeEntry>> {
    let null_pos = raw
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid object format"))?;

    let header = std::str::from_utf8(&raw[..null_pos])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if !header.starts_with("tree ") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Not a tree object: {}", header),
        ));
    }

    let mut entries = Vec::new();
    let mut pos = null_pos + 1;

    while pos < raw.len() {
        let space_pos = raw[pos..]
            .iter()
            .position(|&b| b == b' ')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid tree entry"))?
            + pos;

        let mode = std::str::from_utf8(&raw[pos..space_pos])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .to_string();

        let name_null = raw[space_pos + 1..]
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid tree entry"))?
            + space_pos
            + 1;

        let name = std::str::from_utf8(&raw[space_pos + 1..name_null])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            .to_string();

        let oid_bytes = raw[name_null + 1..name_null + 21].to_vec();

        entries.push(TreeEntry {
            mode,
            name,
            oid: oid_bytes,
        });

        pos = name_null + 21;
    }

    Ok(entries)
}
