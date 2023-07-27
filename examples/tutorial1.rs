#![allow(clippy::single_match)]
#![allow(clippy::redundant_pattern_matching)]

use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(_) = do_some_io() {
        Err("func2 error")?;
    }
    Ok(())
}

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(_) = func2() {
        Err("func1 error")?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    func1()
}
