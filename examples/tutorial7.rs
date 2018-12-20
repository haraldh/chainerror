/*!

~~~bash
$ cargo run -q --example tutorial7
Error: func1 error
caused by: std::io::Error: entity not found
of kind: std::io::ErrorKind::NotFound
The root cause was: std::io::Error: Kind(
    NotFound
)
~~~

!*/

use chainerror::prelude::*;
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
    }
    Ok(())
}