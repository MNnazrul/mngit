use std::io;

use crate::objects::{self, tree};

pub fn run(oid: &str) -> io::Result<()> {
    let raw = objects::read_object(oid)?;
    let entries = tree::parse_tree_entries(&raw)?;

    for entry in entries {
        let oid_hex: String = entry.oid.iter().map(|b| format!("{:02x}", b)).collect();
        let obj_type = if entry.mode == "40000" { "tree" } else { "blob" };
        println!("{:0>6} {} {}\t{}", entry.mode, obj_type, oid_hex, entry.name);
    }

    Ok(())
}
