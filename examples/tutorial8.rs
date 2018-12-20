/*!

~~~bash
$ cargo run -q --example tutorial8
Func1Error: func1 error
Func2Error: Error reading 'foo.txt'
Debug Func2Error:
examples/tutorial8.rs:27: Func2Error(Error reading 'foo.txt')
Caused by:
Kind(NotFound)
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

derive_str_cherr!(Func2Error);

fn func2() -> Result<(), Box<Error>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
    Ok(())
}

derive_str_cherr!(Func1Error);

fn func1() -> Result<(), Box<Error>> {
    func2().map_err(mstrerr!(Func1Error, "func1 error"))?;
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    if let Err(e) = func1() {
        if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
            eprintln!("Func1Error: {}", f1err);

            if let Some(f2err) = f1err.find_cause::<ChainError<Func2Error>>() {
                eprintln!("Func2Error: {}", f2err);
            }

            if let Some(f2err) = f1err.find_chain_cause::<Func2Error>() {
                eprintln!("Debug Func2Error:\n{:?}", f2err);
            }
        }
    }
    Ok(())
}
