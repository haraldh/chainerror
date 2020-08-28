use chainerror::prelude::v1::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

derive_str_cherr!(Func2Error);

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().cherr(Func2Error(format!("Error reading '{}'", filename)))?;
    Ok(())
}

enum Func1ErrorKind {
    Func2,
    IO(String),
}

impl ::std::fmt::Display for Func1ErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Func1ErrorKind::Func2 => write!(f, "func1 error calling func2"),
            Func1ErrorKind::IO(filename) => write!(f, "Error reading '{}'", filename),
        }
    }
}

impl ::std::fmt::Debug for Func1ErrorKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl ::std::error::Error for Func1ErrorKind {}

fn func1() -> ChainResult<(), Func1ErrorKind> {
    func2().cherr(Func1ErrorKind::Func2)?;
    let filename = String::from("bar.txt");
    do_some_io().cherr(Func1ErrorKind::IO(filename))?;
    Ok(())
}

fn handle_func1errorkind(e: &Func1ErrorKind) {
    match e {
        Func1ErrorKind::Func2 => eprintln!("Main Error Report: func1 error calling func2"),
        Func1ErrorKind::IO(ref filename) => {
            eprintln!("Main Error Report: func1 error reading '{}'", filename)
        }
    }
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func1() {
        match *e {
            Func1ErrorKind::Func2 => eprintln!("Main Error Report: func1 error calling func2"),
            Func1ErrorKind::IO(ref filename) => {
                eprintln!("Main Error Report: func1 error reading '{}'", filename)
            }
        }

        handle_func1errorkind(&e);

        if let Some(e) = e.find_chain_cause::<Func2Error>() {
            eprintln!("\nError reported by Func2Error: {}", e)
        }

        eprintln!("\nDebug Error:\n{:?}", e);
    }
    Ok(())
}
