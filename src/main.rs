use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};

#[derive(Parser, Debug)]
struct Args {
    action: String,

    /// For `hash-object`: pass `-w` to write object.
    #[arg(short = 'w', long = "write", default_value_t = false)]
    write: bool,

    /// For `hash-object` : file path
    file: Option<PathBuf>,
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

fn build_blob_raw(data: &[u8]) -> Vec<u8> {
    let mut raw = Vec::with_capacity(32 + data.len());
    raw.extend_from_slice(format!("blob {}\0", data.len()).as_bytes());
    raw.extend_from_slice(data);
    raw
}

fn hash_object(file_path: &Path, write: bool) -> io::Result<String> {
    let data: Vec<u8> = fs::read(file_path)?;
    let raw = build_blob_raw(&data);

    let oid = Sha1::digest(&raw);
    let oid_hex: String = oid.iter().map(|b| format!("{:02x}", b)).collect();

    if write {
        write_object(&oid_hex, &raw)?;
    }

    Ok(oid_hex)
}

fn write_object(oid: &str, raw: &[u8]) -> io::Result<()> {
    // Git stores objects as .git/objects/xx/yyyyyy...
    // where xx is first 2 chars of hash, yyyyyy is rest
    let dir_name = &oid[..2];
    let file_name = &oid[2..];
    let obj_dir = Path::new(".mngit").join("objects").join(dir_name);
    let obj_path = obj_dir.join(file_name);

    // Create directory if it doesn't exist
    fs::create_dir_all(&obj_dir)?;

    // Compress with zlib and write
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    io::Write::write_all(&mut encoder, raw)?;
    let compressed = encoder.finish()?;

    fs::write(obj_path, compressed)?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.action.as_str() {
        "init" => {
            if let Err(e) = init_repo() {
                eprintln!("init failed: {e}");
                std::process::exit(1);
            }
        }

        "hash-object" => {
            let repo_dir = Path::new(".mngit");
            if !repo_dir.exists() || !repo_dir.is_dir() {
                eprintln!("Please first init the repository");
                std::process::exit(1);
            }

            let file_path = match args.file.as_deref() {
                Some(p) => p,
                None => {
                    eprintln!("Usage: hash-object [-w] <file>");
                    std::process::exit(1);
                }
            };

            match hash_object(file_path, args.write) {
                Ok(oid) => println!("{oid}"),
                Err(e) => {
                    eprintln!("hash-object failed: {e}");
                    std::process::exit(1);
                }
            }
        }

        other => {
            eprintln!("No a correct command : {other}");
            eprintln!("Try: init | hash-object [-w] <file>");
            std::process::exit(1);
        }
    }
}
