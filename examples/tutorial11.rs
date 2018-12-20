use chainerror::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<Error>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

derive_str_cherr!(Func2Error);

fn func2() -> Result<(), Box<Error>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
    Ok(())
}

enum Func1Error {
    Func2,
    IO(String),
}

impl ::std::fmt::Display for Func1Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Func1Error::Func2 => write!(f, "func1 error calling func2"),
            Func1Error::IO(filename) => write!(f, "Error reading '{}'", filename),
        }
    }
}

impl ::std::fmt::Debug for Func1Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn func1() -> ChainResult<(), Func1Error> {
    func2().map_err(|e| cherr!(e, Func1Error::Func2))?;
    let filename = String::from("bar.txt");
    do_some_io().map_err(|e| cherr!(e, Func1Error::IO(filename)))?;
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    if let Err(e) = func1() {
        match e.kind() {
            Func1Error::Func2 => eprintln!("Main Error Report: func1 error calling func2"),
            Func1Error::IO(filename) => {
                eprintln!("Main Error Report: func1 error reading '{}'", filename)
            }
        }

        if let Some(e) = e.find_chain_cause::<Func2Error>() {
            eprintln!("Error reported by Func2Error: {}", e)
        }

        eprintln!("\nDebug Error:\n{:?}", e);
    }
    Ok(())
}
