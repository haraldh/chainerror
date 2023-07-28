#![allow(clippy::single_match)]
#![allow(clippy::redundant_pattern_matching)]

pub mod mycrate {
    use chainerror::{Context as _, ErrorDown as _};

    use std::io;

    fn do_some_io(_f: &str) -> std::result::Result<(), io::Error> {
        Err(io::Error::from(io::ErrorKind::NotFound))?;
        Ok(())
    }

    chainerror::str_context!(Func2Error);

    fn func2() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filename = "foo.txt";
        do_some_io(filename).context(Func2Error(format!("Error reading '{}'", filename)))?;
        Ok(())
    }

    #[derive(Debug, Clone)]
    pub enum ErrorKind {
        Func2,
        IO(String),
        FatalError(String),
        Unknown,
    }

    chainerror::err_kind!(Error, ErrorKind);
    pub type Result<T> = std::result::Result<T, Error>;

    impl std::fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorKind::FatalError(e) => write!(f, "fatal error {}", e),
                ErrorKind::Unknown => write!(f, "unknown error"),
                ErrorKind::Func2 => write!(f, "func1 error calling func2"),
                ErrorKind::IO(filename) => write!(f, "Error reading '{}'", filename),
            }
        }
    }

    impl ErrorKind {
        fn from_io_error(e: &io::Error, f: String) -> Self {
            match e.kind() {
                io::ErrorKind::BrokenPipe => panic!("Should not happen"),
                io::ErrorKind::ConnectionReset => {
                    ErrorKind::FatalError(format!("While reading `{}`: {}", f, e))
                }
                _ => ErrorKind::IO(f),
            }
        }
    }

    impl From<&(dyn std::error::Error + 'static + Send + Sync)> for ErrorKind {
        fn from(e: &(dyn std::error::Error + 'static + Send + Sync)) -> Self {
            if let Some(_) = e.downcast_ref::<io::Error>() {
                ErrorKind::IO(String::from("Unknown filename"))
            } else if let Some(_) = e.downcast_inner_ref::<Func2Error>() {
                ErrorKind::Func2
            } else {
                ErrorKind::Unknown
            }
        }
    }

    impl From<&std::boxed::Box<dyn std::error::Error + 'static + Send + Sync>> for ErrorKind {
        fn from(e: &std::boxed::Box<dyn std::error::Error + 'static + Send + Sync>) -> Self {
            Self::from(&**e)
        }
    }

    impl From<&Func2Error> for ErrorKind {
        fn from(_: &Func2Error) -> Self {
            ErrorKind::Func2
        }
    }

    impl From<&io::Error> for ErrorKind {
        fn from(e: &io::Error) -> Self {
            ErrorKind::IO(format!("{}", e))
        }
    }

    pub fn func1() -> Result<()> {
        func2().map_err(|e| ErrorKind::from(&e))?;

        let filename = "bar.txt";

        do_some_io(filename).map_context(|e| ErrorKind::from_io_error(e, filename.into()))?;
        do_some_io(filename).map_context(|_| ErrorKind::IO(filename.into()))?;
        do_some_io(filename).map_context(|e| ErrorKind::from(e))?;

        Ok(())
    }

    pub fn super_func1() -> Result<()> {
        func1().map_context(|e| ErrorKind::from(e))?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use mycrate::super_func1;
    use mycrate::ErrorKind;
    use std::error::Error;
    use std::io;

    if let Err(e) = super_func1() {
        match e.kind() {
            ErrorKind::FatalError(f) => eprintln!("Main Error Report: Fatal Error: {}", f),
            ErrorKind::Unknown => eprintln!("Main Error Report: Unknown error occurred"),
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
