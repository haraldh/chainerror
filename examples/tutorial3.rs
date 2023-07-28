use chainerror::Context as _;

use std::error::Error;
use std::io;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    do_some_io().context("func2 error")?;
    Ok(())
}

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    func2().context("func1 error")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func1() {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
    Ok(())
}
