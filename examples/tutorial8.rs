use chainerror::{Context as _, ErrorDown as _};

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

chainerror::str_context!(Func1Error);

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    func2().context(Func1Error("func1 error".to_string()))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = func1() {
        if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
            eprintln!("Func1Error: {}", f1err);

            if let Some(f2err) = f1err.find_cause::<chainerror::Error<Func2Error>>() {
                eprintln!("Func2Error: {}", f2err);
            }

            if let Some(f2err) = f1err.find_chain_cause::<Func2Error>() {
                eprintln!("Debug Func2Error:\n{:?}", f2err);
            }
        }
        std::process::exit(1);
    }
    Ok(())
}
