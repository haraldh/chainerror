use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<Error>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<Error>> {
    if let Err(_) = do_some_io() {
        Err("func2 error")?;
    }
    Ok(())
}

fn func1() -> Result<(), Box<Error>> {
    if let Err(_) = func2() {
        Err("func1 error")?;
    }
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    func1()
}
