mod commands;
mod objects;

use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    action: String,

    #[arg(short = 'w', long = "write", default_value_t = false)]
    write: bool,

    #[arg(short = 'p', long = "print", default_value_t = false)]
    print: bool,

    file: Option<std::path::PathBuf>,

    oid: Option<String>,
}

fn require_repo() {
    let repo_dir = Path::new(".mngit");
    if !repo_dir.exists() || !repo_dir.is_dir() {
        eprintln!("Please first init the repository");
        std::process::exit(1);
    }
}

fn main() {
    let args = Args::parse();

    match args.action.as_str() {
        "init" => {
            if let Err(e) = commands::init::init_repo() {
                eprintln!("init failed: {e}");
                std::process::exit(1);
            }
        }

        "hash-object" => {
            require_repo();
            let file_path = match args.file.as_deref() {
                Some(p) => p,
                None => {
                    eprintln!("Usage: hash-object [-w] <file>");
                    std::process::exit(1);
                }
            };

            match commands::hash_object::run(file_path, args.write) {
                Ok(oid) => println!("{oid}"),
                Err(e) => {
                    eprintln!("hash-object failed: {e}");
                    std::process::exit(1);
                }
            }
        }

        "cat-file" => {
            require_repo();
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

            if let Err(e) = commands::cat_file::run(oid) {
                eprintln!("cat-file failed: {e}");
                std::process::exit(1);
            }
        }

        "write-tree" => {
            require_repo();
            match commands::write_tree::run() {
                Ok(tree_oid) => println!("{tree_oid}"),
                Err(e) => {
                    eprintln!("write-tree failed: {e}");
                    std::process::exit(1);
                }
            }
        }

        "ls-tree" => {
            require_repo();
            let oid = match args.oid.as_deref() {
                Some(o) => o,
                None => match args.file.as_deref() {
                    Some(p) => p.to_str().unwrap_or(""),
                    None => {
                        eprintln!("Usage: ls-tree <oid>");
                        std::process::exit(1);
                    }
                },
            };

            if let Err(e) = commands::ls_tree::run(oid) {
                eprintln!("ls-tree failed: {e}");
                std::process::exit(1);
            }
        }

        other => {
            eprintln!("No a correct command : {other}");
            eprintln!(
                "Try: init | hash-object [-w] <file> | cat-file -p <oid> | write-tree | ls-tree <oid>"
            );
            std::process::exit(1);
        }
    }
}
