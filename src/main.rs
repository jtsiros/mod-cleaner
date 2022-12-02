use std::{env, process, fs, io, collections::HashMap};

use walkdir::{WalkDir};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Error: you must supply a directory path like: C:\\Users\\sims4\\mods");
        process::exit(exitcode::NOINPUT);
    }
    let mod_path = &args[1];
    println!("Searching for duplicate mods in: {mod_path}");
    
    // the total number of files removed.
    let mut total = 0;
    let mut file_names = HashMap::new();

    let walker = WalkDir::new(mod_path).into_iter();
    for entry in walker.filter_map(|e| e.ok()) {

        if entry.path().is_dir() {
            println!("looking in dir: {}", entry.path().display());
        }

        let f_name = entry.file_name().to_string_lossy().to_string();

        if is_mod_file(&f_name) {
            match file_names.get(&f_name) {
                Some(_) => {
                    println!("duplicate file :: {}", f_name);
                    fs::remove_file(entry.path()).expect("failed to delete file.");
                    total += 1;
                },
                _ => {
                    file_names.insert(f_name, ());
                }
            }
        }
    }

    if total > 0 {
        println!("{} file(s) removed.", total);
    } else {
        println!("None found.");
    }

    Ok(())
}

fn is_mod_file(fname: &str) -> bool {
    fname.ends_with(".package")
}