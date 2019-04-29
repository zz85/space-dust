use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::env;

/**
 * compile: rustc du.rs
 * run: compare on mac
 * /usr/bin/time -lp ./du ~/Documents
 * /usr/bin/time -lp du -sk ~/Documents
 */

fn visit_dirs(dir: &Path) -> io::Result<u64> {
    if !dir.is_file() && !dir.is_dir()  {
        return Ok(0);
    }
    let metadata = match dir.metadata() {
        Ok(metadata) => metadata,
        Err(e) => {
            println!("error reading path {} - Reason {}", dir.display(), e);
            return Ok(0)
        }
    };

    let mut size = metadata.len();
    if metadata.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            let child_size = visit_dirs(&path)?;
            size += child_size;
        }
    }

    Ok(size)
}

fn friendly(bytes:u64) -> String  {
    return format!("{:>8.3}K", bytes as f64 / 1024.0);
}

fn main() {
    let args:Vec<String> =  env::args().collect();
    let len = args.len();
    let path = if len > 1 { &args[1] } else { "./" };
    println!("path {}", path);
    println!("args: {:?}, len: {}", args, args.len());
    let size = visit_dirs(Path::new(path)).unwrap();

    println!("Ok done {}", friendly(size));
}