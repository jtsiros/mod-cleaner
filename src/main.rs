use std::{env, process, fs::{self, File}, io, collections::HashMap};

use walkdir::{WalkDir};


// Mod Cleaner is a mod cleaner tool responsible for deleting
// duplicate mod files for the Sims 4. This could also be used for other
// generic file deduping purposes. Simply provide the directory
// you want to dedup:
//
// mod-cleaner /path/to/dir
//
// This will return the number of files that were successfully deleted.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Error: you must supply a directory path like: C:\\Users\\sims4\\mods");
        process::exit(exitcode::NOINPUT);
    }
    let mod_path = &args[1];
    println!("Searching for duplicate mods in: {mod_path}");
    
    // the total number of files removed.
    match dedup_files(mod_path) {
        Ok(num_files) => println!("{num_files} file(s) removed."),
        Err(e) => eprintln!("error: {e}"),
    };
}

fn dedup_files(mod_path: &str) -> Result<u32, io::Error> {

    let mut total: u32 = 0;
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
                    total += 1u32;
                },
                _ => {
                    file_names.insert(f_name, ());
                }
            }
        }
    }

    Ok(total)

}

fn is_mod_file(fname: &str) -> bool {
    fname.ends_with(".package")
}

#[test]
fn delete_dup_files() -> Result<(), io::Error> {
    File::create("testdata/moddir2/[D1] my mod.package")?;
    File::create("testdata/[D1] my mod.package")?;
    let total = dedup_files("testdata").ok().unwrap();
    assert_eq!(2, total, "expected 2 dup files to be deleted but got {}", total);
    Ok(())
}