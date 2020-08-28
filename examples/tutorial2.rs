use chainerror::prelude::v1::*;

use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = do_some_io() {
        Err(e).cherr("func2 error")?;
    }
    Ok(())
}

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func2() {
        Err(e).cherr("func1 error")?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    func1()
}
