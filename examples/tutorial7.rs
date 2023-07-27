#![allow(clippy::single_match)]
#![allow(clippy::redundant_pattern_matching)]

use chainerror::prelude::v1::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().context(format!("Error reading '{}'", filename))?;
    Ok(())
}

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    func2().context("func1 error")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func1() {
        eprintln!("Error: {}", e);
        if let Some(s) = e.downcast_chain_ref::<String>() {
            if let Some(ioerror) = s.find_cause::<io::Error>() {
                eprintln!("caused by: std::io::Error: {}", ioerror);
                match ioerror.kind() {
                    io::ErrorKind::NotFound => eprintln!("of kind: std::io::ErrorKind::NotFound"),
                    _ => {}
                }
            }

            if let Some(e) = s.root_cause() {
                let ioerror = e.downcast_ref::<io::Error>().unwrap();
                eprintln!("The root cause was: std::io::Error: {:#?}", ioerror);
            }
        }
        std::process::exit(1);
    }
    Ok(())
}
