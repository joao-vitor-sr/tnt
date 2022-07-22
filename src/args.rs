pub struct Args {
    pub note: Option<String>,
    pub remove: bool,
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
            Short('h') | Long("help") => {
                println!("Usage: tnt [-r|--remove] NOTE");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args { note, remove })
}
