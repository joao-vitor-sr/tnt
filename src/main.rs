use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    process::Command,
};

use cli::{input_yn, list_files};
use output::{print_green, print_red};

use crate::cli::parse_args;

pub mod cli;
mod output;

fn create_folder(path: &Path) -> io::Result<()> {
    fs::create_dir(&path)?;
    let mut perms = fs::metadata(&path)?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(&path, perms)?;
    Ok(())
}

fn return_notes_folder() -> io::Result<PathBuf> {
    match env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let mut path = PathBuf::from(val);
            path.push("tnt");
            if !path.exists() {
                create_folder(&path)?;
            }
            return Ok(path);
        }
        Err(_) => {
            let path = PathBuf::from("/usr/share/tnt");
            if !Path::new(&path).exists() {
                create_folder(&path)?;
            }
            return Ok(path);
        }
    };
}

fn create_note(note_path: &Path, confirm: bool) -> io::Result<bool> {
    if confirm && !input_yn("\nThis note doesn't exist, do you want to create it? [Y/n]")? {
        return Ok(false);
    }
    File::create(note_path)?;
    Ok(true)
}

fn edit_note(note_path: &Path) -> io::Result<()> {
    if !note_path.exists() && !create_note(&note_path, true)? {
        return Err(io::Error::new(
            ErrorKind::Other,
            "Note path 
            doesn't exist! Try creating a folder called
            \"tnt\" in the XDG_DATA_HOME or in the /usr/share",
        ));
    }
    Command::new(env::var("EDITOR").unwrap())
        .arg(&note_path)
        .spawn()?
        .wait()?;
    println!("Edit note");
    Ok(())
}

fn return_note_filename(note: &String) -> String {
    let mut final_note_filename = note.clone().replace(" ", "_").to_lowercase();
    final_note_filename.push_str(".md");
    final_note_filename
}

fn main() {
    let args = parse_args().expect("Failed to read args");
    let mut notes_path = return_notes_folder().expect("Failed to get notes path");

    if args.note.is_none() {
        list_files(&notes_path).expect("No valid command to list directory contents");
        return;
    }

    let note = args.note.expect("Failed to get the note");
    let note_filename = return_note_filename(&note);

    notes_path.push(note_filename);

    print_green(&note[..]);
    edit_note(&notes_path.as_path());
}
