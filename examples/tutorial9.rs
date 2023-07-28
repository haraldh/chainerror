use chainerror::{Context as _, ErrorDown};

use std::error::Error;
use std::io;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

chainerror::str_context!(Func2Error);

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().context(Func2Error(format!("Error reading '{}'", filename)))?;
    Ok(())
}

chainerror::str_context!(Func1ErrorFunc2);
chainerror::str_context!(Func1ErrorIO);

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    func2().context(Func1ErrorFunc2::new("func1 error calling func2"))?;
    let filename = "bar.txt";
    do_some_io().context(Func1ErrorIO(format!("Error reading '{}'", filename)))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func1() {
        if let Some(s) = e.downcast_ref::<chainerror::Error<Func1ErrorIO>>() {
            eprintln!("Func1ErrorIO:\n{:?}", s);
        }

        if let Some(s) = e.downcast_chain_ref::<Func1ErrorFunc2>() {
            eprintln!("Func1ErrorFunc2:\n{:?}", s);
        }
        std::process::exit(1);
    }
    Ok(())
}
