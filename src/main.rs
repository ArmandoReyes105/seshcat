use std::io;
use std::path::{Path, PathBuf};
use std::process;
use std::{env, fs};

fn main() {
    let target = resolve_target_path();

    match list_dir(&target) {
        Ok(paths) => print_entries(&paths),
        Err(error) => {
            eprintln!("Error al leer '{}': {error}", target.display());
            process::exit(1);
        }
    }
    println!("Explorando: {}", target.display());
}

fn resolve_target_path() -> PathBuf {
    match env::args().nth(1) {
        Some(path_str) => PathBuf::from(path_str),
        None => PathBuf::from("."),
    }
}

fn list_dir(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let dir_entries: Vec<fs::DirEntry> = entries.collect::<io::Result<Vec<_>>>()?;

    let paths: Vec<PathBuf> = dir_entries.iter().map(|entry| entry.path()).collect();

    Ok(paths)
}

fn print_entries(paths: &[PathBuf]) {
    for path in paths {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| path.to_string_lossy());

        let kind = if path.is_dir() {
            "dir"
        } else if path.is_file() {
            "file"
        } else {
            "other"
        };

        println!("[{kind}] {name}");
    }
}
