use std::{fs, io, path::Path};

use crate::objects::{self, blob, hex_to_bytes, tree::TreeEntry};

pub fn run() -> io::Result<String> {
    write_tree_at_path(Path::new("."))
}

fn write_tree_at_path(path: &Path) -> io::Result<String> {
    let mut entries = Vec::new();

    let mut dir_entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            name_str != ".mngit"
                && name_str != "target"
                && name_str != ".git"
                && name_str != "node_modules"
                && !name_str.starts_with('.')
        })
        .collect();

    dir_entries.sort_by_key(|e| e.file_name());

    for entry in dir_entries {
        let entry_path = entry.path();
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid file name"))?
            .to_string();

        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            let sub_tree_oid = write_tree_at_path(&entry_path)?;
            let oid_bytes = hex_to_bytes(&sub_tree_oid)?;

            entries.push(TreeEntry {
                mode: "40000".to_string(),
                name,
                oid: oid_bytes,
            });
        } else if metadata.is_file() {
            let file_oid = blob::hash_object(&entry_path, true)?;
            let oid_bytes = hex_to_bytes(&file_oid)?;

            #[cfg(unix)]
            let mode = {
                use std::os::unix::fs::PermissionsExt;
                let perms = metadata.permissions();
                if perms.mode() & 0o111 != 0 {
                    "100755"
                } else {
                    "100644"
                }
            };

            #[cfg(not(unix))]
            let mode = "100644";

            entries.push(TreeEntry {
                mode: mode.to_string(),
                name,
                oid: oid_bytes,
            });
        }
    }

    let raw = objects::tree::build_tree_raw(&entries);
    objects::hash_and_store(&raw)
}
