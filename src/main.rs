use std::{fs, io, path::Path};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    action: String,
}

fn init_repo() -> io::Result<()> {
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

fn main() {
    let args = Args::parse();

    if args.action == "init" && let Err(e) = init_repo(){
        eprintln!("init failed: {e}");
        std::process::exit(1);
    }
}
