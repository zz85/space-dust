use std::env;
use walkdir::WalkDir;

fn friendly(bytes:u64) -> String  {
    return format!("{:>8.3}K", bytes as f64 / 1024.0);
}

fn main() {
    let args:Vec<String> =  env::args().collect();
    let len = args.len();
    let path = if len > 1 { &args[1] } else { "./" };
    println!("Scanning path {}...", path);

    // for entry in WalkDir::new(path) {
    //     let entry = entry.unwrap();
    //     println!("path: {}", entry.path().display());
    // }

    let mut size:u64 = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        // println!("{}", entry.path().display());
        match entry.metadata() {
            Ok(meta) => {
                size += meta.len()
                // println!("entry {:?}", )
            },
            Err(_) => {}
        };

    }

    println!("Space scan done: {}", friendly(size));
}