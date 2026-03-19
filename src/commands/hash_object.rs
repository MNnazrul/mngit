use std::{io, path::Path};

use crate::objects::blob;

pub fn run(file_path: &Path, write: bool) -> io::Result<String> {
    blob::hash_object(file_path, write)
}
