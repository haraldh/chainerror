use chainerror::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<Error>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<Error>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
    Ok(())
}

fn func1() -> Result<(), Box<Error>> {
    func2().map_err(mstrerr!("func1 error"))?;
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    if let Err(e) = func1() {
        eprintln!("Error: {}", e);
        let mut s = e.as_ref();
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
    }
    Ok(())
}
