use std::error::Error;
use std::io;
use std::result::Result;

use chainerror::prelude::v1::*;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func3() -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().context(format!("Error reading '{}'", filename))?;
    Ok(())
}

derive_str_context!(Func2Error);

fn func2() -> ChainResult<(), Func2Error> {
    func3().context(Func2Error("func2 error: calling func3".to_string()))?;
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
    func2().context(Func1Error::Func2)?;
    let filename = String::from("bar.txt");
    do_some_io().context(Func1Error::IO(filename))?;
    Ok(())
}

fn main() {
    if let Err(e) = func1() {
        eprintln!("\nDisplay Error {{}}:\n{}", e);

        eprintln!("\nAlternative Display Error {{:#}}:\n{:#}", e);

        eprintln!("\nDebug Error {{:?}}:\n{:?}", e);

        eprintln!("\nAlternative Debug Error {{:#?}}:\n{:#?}\n", e);

        match e.kind() {
            Func1Error::Func2 => eprintln!("Main Error Report: func1 error calling func2"),
            Func1Error::IO(filename) => {
                eprintln!("Main Error Report: func1 error reading '{}'", filename)
            }
        }

        if let Some(e) = e.find_chain_cause::<Func2Error>() {
            eprintln!("\nError reported by Func2Error: {}", e)
        }

        if let Some(e) = e.root_cause() {
            let ioerror = e.downcast_ref::<io::Error>().unwrap();
            eprintln!("\nThe root cause was: std::io::Error: {:#?}", ioerror);
        }
    }
}
