pub mod mycrate {
    use chainerror::prelude::v1::*;
    use std::io;

    fn do_some_io() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err(io::Error::from(io::ErrorKind::NotFound))?;
        Ok(())
    }

    derive_str_context!(Func2Error);

    fn func2() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filename = "foo.txt";
        do_some_io().context(Func2Error(format!("Error reading '{}'", filename)))?;
        Ok(())
    }

    #[derive(Debug, Clone)]
    pub enum ErrorKind {
        Func2,
        IO(String),
    }

    derive_err_kind!(Error, ErrorKind);

    pub type Result<T> = std::result::Result<T, Error>;

    impl std::fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorKind::Func2 => write!(f, "func1 error calling func2"),
                ErrorKind::IO(filename) => write!(f, "Error reading '{}'", filename),
            }
        }
    }

    pub fn func1() -> Result<()> {
        func2().context(ErrorKind::Func2)?;
        let filename = String::from("bar.txt");
        do_some_io().context(ErrorKind::IO(filename))?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use mycrate::func1;
    use mycrate::ErrorKind;
    use std::error::Error;
    use std::io;

    if let Err(e) = func1() {
        match e.kind() {
            ErrorKind::Func2 => eprintln!("Main Error Report: func1 error calling func2"),
            ErrorKind::IO(ref filename) => {
                eprintln!("Main Error Report: func1 error reading '{}'", filename)
            }
        }

        eprintln!();
        let mut s: &dyn Error = &e;
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

        eprintln!("\nDebug Error:\n{:?}", e);

        std::process::exit(1);
    }
    Ok(())
}
