use std::io;

use crate::objects;

pub fn run(oid: &str) -> io::Result<()> {
    let raw = objects::read_object(oid)?;

    let null_pos = raw
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid object format"))?;

    let content = &raw[null_pos + 1..];
    io::Write::write_all(&mut io::stdout(), content)?;

    Ok(())
}
