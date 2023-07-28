use chainerror::Context as _;

use std::error::Error;
use std::io;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = do_some_io() {
        Err(e).context("func2 error")?;
    }
    Ok(())
}

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func2() {
        Err(e).context("func1 error")?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    func1()
}
