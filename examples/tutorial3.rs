use chainerror::*;

use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    do_some_io().map_err(|e| cherr!(e, "func2 error"))?;
    Ok(())
}

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    func2().map_err(|e| cherr!(e, "func1 error"))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func1() {
        eprintln!("{:?}", e);
    }
    Ok(())
}
