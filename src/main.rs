use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use output::{print_green, print_red};

use crate::args::parse_args;

pub mod args;
mod output;

fn get_notes_folder_path() -> io::Result<PathBuf> {
    match env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let mut path = PathBuf::from(val);
            path.push("tnt");
            if !path.exists() {
                fs::create_dir(&path)?;
            }
            return Ok(path);
        }
        Err(_) => {
            let path = PathBuf::from("/usr/share/tnt");
            if !Path::new(&path).exists() {
                fs::create_dir(&path)?;
            }
            return Ok(path);
        }
    };
}

fn list_notes(path: &Path) -> Result<(), String> {
    for file in fs::read_dir(path).unwrap() {
        println!(
            "{}",
            file.unwrap()
                .file_name()
                .to_str()
                .expect("Failed to transform OsStr into str")
        );
    }
    Ok(())
}

fn main() {
    let args = parse_args().expect("Failed to read args");
    let notes_folder_path = get_notes_folder_path().expect("Failed to get notes path");

    if args.note.is_none() {
        list_notes(&notes_folder_path).expect("No valid command to list directory contents");
        return;
    }

    println!("aeou");
}
