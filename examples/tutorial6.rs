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
        let mut s: &(dyn Error) = e.as_ref();
        while let Some(c) = s.source() {
            if let Some(ioerror) = c.downcast_ref::<io::Error>() {
                eprintln!("caused by: std::io::Error: {}", ioerror);
                match ioerror.kind() {
                    io::ErrorKind::NotFound => eprintln!("of kind: std::io::ErrorKind::NotFound"),
                    _ => {}
                }
            } else {
                eprintln!("caused by: {}", c);
            }
            s = c;
        }
        std::process::exit(1);
    }
    Ok(())
}
