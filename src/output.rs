use ansi_term::Colour::{Green, Red};

pub fn print_green(msg: &str) {
    println!("{}", Green.paint(msg));
}

pub fn print_red(msg: &str) {
    println!("{}", Red.paint(msg));
}
