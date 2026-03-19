use std::{fs, io, path::Path};

use sha1::{Digest, Sha1};

use super::write_object;

pub fn build_blob_raw(data: &[u8]) -> Vec<u8> {
    let mut raw = Vec::with_capacity(32 + data.len());
    raw.extend_from_slice(format!("blob {}\0", data.len()).as_bytes());
    raw.extend_from_slice(data);
    raw
}

pub fn hash_object(file_path: &Path, write: bool) -> io::Result<String> {
    let data: Vec<u8> = fs::read(file_path)?;
    let raw = build_blob_raw(&data);

    let oid = Sha1::digest(&raw);
    let oid_hex: String = oid.iter().map(|b| format!("{:02x}", b)).collect();

    if write {
        write_object(&oid_hex, &raw)?;
    }

    Ok(oid_hex)
}
