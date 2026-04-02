pub mod blob;
pub mod commit;
pub mod tree;

use std::{fs, io, path::Path};

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};

pub fn write_object(oid: &str, raw: &[u8]) -> io::Result<()> {
    let dir_name = &oid[..2];
    let file_name = &oid[2..];
    let obj_dir = Path::new(".mngit").join("objects").join(dir_name);
    let obj_path = obj_dir.join(file_name);

    fs::create_dir_all(&obj_dir)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    io::Write::write_all(&mut encoder, raw)?;
    let compressed = encoder.finish()?;

    fs::write(obj_path, compressed)?;
    Ok(())
}

pub fn read_object(oid: &str) -> io::Result<Vec<u8>> {
    let dir_name = &oid[..2];
    let file_name = &oid[2..];
    let obj_path = Path::new(".mngit")
        .join("objects")
        .join(dir_name)
        .join(file_name);

    if !obj_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Object not found: {}", oid),
        ));
    }

    let compressed = fs::read(obj_path)?;
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut raw = Vec::new();
    io::Read::read_to_end(&mut decoder, &mut raw)?;

    Ok(raw)
}

pub fn hex_to_bytes(hex: &str) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid hex: {}", e))
        })?;
        bytes.push(byte);
    }
    Ok(bytes)
}

pub fn hash_and_store(raw: &[u8]) -> io::Result<String> {
    let oid = Sha1::digest(raw);
    let oid_hex: String = oid.iter().map(|b| format!("{:02x}", b)).collect();
    write_object(&oid_hex, raw)?;
    Ok(oid_hex)
}
