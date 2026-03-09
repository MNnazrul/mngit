use std::{
    fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};

#[derive(Debug, Clone)]
struct TreeEntry {
    mode: String,
    name: String,
    oid: Vec<u8>, // 20 raw bytes
}

#[derive(Parser, Debug)]
struct Args {
    action: String,

    /// For `hash-object`: pass `-w` to write object.
    #[arg(short = 'w', long = "write", default_value_t = false)]
    write: bool,

    /// For `cat-file`: pass `-p` to pretty-print object.
    #[arg(short = 'p', long = "print", default_value_t = false)]
    print: bool,

    /// For `hash-object`: file path. For `cat-file`: object id.
    file: Option<PathBuf>,

    /// For `cat-file`: object id (alternative position)
    oid: Option<String>,
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

fn read_object(oid: &str) -> io::Result<Vec<u8>> {
    // Read object from .mngit/objects/xx/yyyyyy...
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

    // Read and decompress
    let compressed = fs::read(obj_path)?;
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut raw = Vec::new();
    io::Read::read_to_end(&mut decoder, &mut raw)?;

    Ok(raw)
}

fn cat_file(oid: &str) -> io::Result<()> {
    let raw = read_object(oid)?;

    // Find the null byte that separates header from content
    let null_pos = raw
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid object format"))?;

    // Extract content after the null byte
    let content = &raw[null_pos + 1..];

    // Print content (write to stdout)
    io::Write::write_all(&mut io::stdout(), content)?;

    Ok(())
}

fn build_tree_raw(entries: &[TreeEntry]) -> Vec<u8> {
    // Build tree content (without header)
    let mut content = Vec::new();

    for entry in entries {
        // Format: "<mode> <name>\0<20-byte-oid>"
        content.extend_from_slice(entry.mode.as_bytes());
        content.push(b' ');
        content.extend_from_slice(entry.name.as_bytes());
        content.push(0);
        content.extend_from_slice(&entry.oid);
    }

    // Add header
    let mut raw = Vec::with_capacity(32 + content.len());
    raw.extend_from_slice(format!("tree {}\0", content.len()).as_bytes());
    raw.extend_from_slice(&content);
    raw
}

fn write_tree_at_path(path: &Path) -> io::Result<String> {
    let mut entries = Vec::new();

    // Read directory entries
    let mut dir_entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            // Skip .mngit, target, .git, and other common build/cache directories
            name_str != ".mngit"
                && name_str != "target"
                && name_str != ".git"
                && name_str != "node_modules"
                && !name_str.starts_with('.')
        })
        .collect();

    // Sort by name (Git requirement)
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
            // Recursively create tree for subdirectory
            let sub_tree_oid = write_tree_at_path(&entry_path)?;
            let oid_bytes = hex_to_bytes(&sub_tree_oid)?;

            entries.push(TreeEntry {
                mode: "40000".to_string(), // directory mode in Git
                name,
                oid: oid_bytes,
            });
        } else if metadata.is_file() {
            // Create blob for file
            let file_oid = hash_object(&entry_path, true)?;
            let oid_bytes = hex_to_bytes(&file_oid)?;

            // Determine file mode (simplified: executable or regular)
            #[cfg(unix)]
            let mode = {
                use std::os::unix::fs::PermissionsExt;
                let perms = metadata.permissions();
                if perms.mode() & 0o111 != 0 {
                    "100755" // executable
                } else {
                    "100644" // regular file
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

    // Build tree object
    let raw = build_tree_raw(&entries);

    // Calculate OID
    let oid = Sha1::digest(&raw);
    let oid_hex: String = oid.iter().map(|b| format!("{:02x}", b)).collect();

    // Write tree object
    write_object(&oid_hex, &raw)?;

    Ok(oid_hex)
}

fn write_tree() -> io::Result<String> {
    write_tree_at_path(Path::new("."))
}

fn hex_to_bytes(hex: &str) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid hex: {}", e))
        })?;
        bytes.push(byte);
    }
    Ok(bytes)
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

        "cat-file" => {
            let repo_dir = Path::new(".mngit");
            if !repo_dir.exists() || !repo_dir.is_dir() {
                eprintln!("Please first init the repository");
                std::process::exit(1);
            }

            if !args.print {
                eprintln!("Usage: cat-file -p <oid>");
                std::process::exit(1);
            }

            let oid = match args.oid.as_deref() {
                Some(o) => o,
                None => match args.file.as_deref() {
                    Some(p) => p.to_str().unwrap_or(""),
                    None => {
                        eprintln!("Usage: cat-file -p <oid>");
                        std::process::exit(1);
                    }
                },
            };

            if let Err(e) = cat_file(oid) {
                eprintln!("cat-file failed: {e}");
                std::process::exit(1);
            }
        }

        "write-tree" => {
            let repo_dir = Path::new(".mngit");
            if !repo_dir.exists() || !repo_dir.is_dir() {
                eprintln!("Please first init the repository");
                std::process::exit(1);
            }

            match write_tree() {
                Ok(tree_oid) => println!("{tree_oid}"),
                Err(e) => {
                    eprintln!("write-tree failed: {e}");
                    std::process::exit(1);
                }
            }
        }

        other => {
            eprintln!("No a correct command : {other}");
            eprintln!("Try: init | hash-object [-w] <file> | cat-file -p <oid> | write-tree");
            std::process::exit(1);
        }
    }
}
