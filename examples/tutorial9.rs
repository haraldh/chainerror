use chainerror::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

derive_str_cherr!(Func2Error);

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
    Ok(())
}

derive_str_cherr!(Func1ErrorFunc2);
derive_str_cherr!(Func1ErrorIO);

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    func2().map_err(mstrerr!(Func1ErrorFunc2, "func1 error calling func2"))?;
    let filename = "bar.txt";
    do_some_io().map_err(mstrerr!(Func1ErrorIO, "Error reading '{}'", filename))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func1() {
        if let Some(s) = e.downcast_ref::<ChainError<Func1ErrorIO>>() {
            eprintln!("Func1ErrorIO:\n{:?}", s);
        }

        if let Some(s) = e.downcast_chain_ref::<Func1ErrorFunc2>() {
            eprintln!("Func1ErrorFunc2:\n{:?}", s);
        }
    }
    Ok(())
}
