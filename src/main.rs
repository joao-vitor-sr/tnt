use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use output::{print_green, print_red};

use crate::args::parse_args;

pub mod args;
mod output;

fn return_notes_folder() -> io::Result<PathBuf> {
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

fn input_yn(msg: &str) -> io::Result<bool> {
    println!("{msg}");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(&input.to_uppercase().trim()[0..1] == "Y")
}

fn create_note(note_path: &Path, confirm: bool) -> io::Result<bool> {
    if confirm && !input_yn("\nThis note doesn't exist, do you want to create it? [Y/n]")? {
        return Ok(false);
    }
    println!("Create note");
    Ok(true)
}

fn edit_note(note_path: &Path) {
    if !note_path.exists() && !create_note(&note_path, true).expect("Failed to create note") {
        return;
    }
    println!("Edit note");
}

fn return_note_filename(note: &String) -> String {
    let mut final_note_filename = note.clone().replace(" ", "_").to_lowercase();
    final_note_filename.push_str(".md");
    final_note_filename
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
