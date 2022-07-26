use std::path::Path;

use crate::output::print_green;

const HELP: &str = "\
Usage: 
  tnt NOTE [OPTIONS]

META OPTIONS:
    -h, --help          show list of command-line options
    -v, --version       show version of tnt
OPTIONS:
    -r, --remove        remove the defined note
ARGS:
    <NOTE>              input note";

pub struct Args {
    pub note: Option<String>,
    pub remove: bool,
}

pub fn print_version() {
    let message: Option<&str> = option_env!("CARGO_PKG_VERSION");
    print_green(message.unwrap_or("Unknown version"));
    return;
}

pub fn list_files(path: &Path) -> std::io::Result<()> {
    for file in std::fs::read_dir(path).unwrap() {
        println!(
            "{}",
            file?
                .file_name()
                .into_string()
                .expect("Failed to transform note filename into String")
                .replace(".md", "")
        );
    }
    Ok(())
}

pub fn input_yn() -> std::io::Result<bool> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(&input.to_uppercase().trim()[0..1] == "Y")
}

fn print_help() {
    print!("{}", HELP);
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut note = None;
    let mut remove = false;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('r') | Long("remove") => {
                remove = true;
            }
            Value(val) if note.is_none() => {
                note = Some(val.into_string()?);
            }
            Short('v') | Long("version") => {
                print_version();
                std::process::exit(0);
            }
            Short('h') | Long("help") => {
                print_help();
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        note,
        remove,
    })
}
