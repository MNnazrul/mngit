use std::{fs, io, path::Path};

pub fn init_repo() -> io::Result<()> {
    let init_paths = [
        ".mngit",
        ".mngit/objects/",
        ".mngit/refs/heads/",
        ".mngit/HEAD",
    ];

    for p in init_paths {
        let path = Path::new(p);

        if p.ends_with("HEAD") {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            if !path.exists() {
                fs::write(path, b"ref: refs/heads/main\n")?;
            }
        } else {
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}
