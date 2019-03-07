pub mod mycrate {
    use std::error::Error as StdError;

    use func2mod::{do_some_io, func2};

    pub mod func2mod {
        use std::error::Error as StdError;
        use std::io;

        pub enum ErrorKind {
            IO(String),
        }

        impl std::fmt::Display for ErrorKind {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    ErrorKind::IO(s) => std::fmt::Display::fmt(s, f),
                }
            }
        }

        impl std::fmt::Debug for ErrorKind {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    ErrorKind::IO(s) => std::fmt::Display::fmt(s, f),
                }
            }
        }

        macro_rules! mcherr {
            ( $k:expr ) => {{
                |e| {
                    Error(
                        $k,
                        Some(Box::from(e)),
                        Some(concat!(file!(), ":", line!(), ": ")),
                    )
                }
            }};
        }

        pub struct Error(
            ErrorKind,
            Option<Box<dyn std::error::Error + 'static>>,
            Option<&'static str>,
        );

        impl Error {
            pub fn kind(&self) -> &ErrorKind {
                &self.0
            }
        }

        impl From<ErrorKind> for Error {
            fn from(e: ErrorKind) -> Self {
                Error(e, None, None)
            }
        }

        impl std::error::Error for Error {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.1.as_ref().map(|e| e.as_ref())
            }
        }

        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                if let Some(ref o) = self.2 {
                    std::fmt::Display::fmt(o, f)?;
                }

                std::fmt::Debug::fmt(&self.0, f)?;

                if let Some(e) = self.source() {
                    std::fmt::Display::fmt("\nCaused by:\n", f)?;
                    std::fmt::Debug::fmt(&e, f)?;
                }
                Ok(())
            }
        }

        pub fn do_some_io() -> std::result::Result<(), Box<dyn std::error::Error>> {
            Err(io::Error::from(io::ErrorKind::NotFound))?;
            Ok(())
        }

        pub fn func2() -> std::result::Result<(), Error> {
            let filename = "foo.txt";
            do_some_io().map_err(mcherr!(ErrorKind::IO(format!(
                "Error reading '{}'",
                filename
            ))))?;
            Ok(())
        }
    }

    #[derive(Debug)]
    pub enum ErrorKind {
        Func2,
        IO(String),
    }

    impl std::fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ErrorKind::Func2 => write!(f, "func1 error calling func2"),
                ErrorKind::IO(filename) => write!(f, "Error reading '{}'", filename),
            }
        }
    }

    macro_rules! mcherr {
        ( $k:expr ) => {{
            |e| {
                Error(
                    $k,
                    Some(Box::from(e)),
                    Some(concat!(file!(), ":", line!(), ": ")),
                )
            }
        }};
    }

    pub struct Error(
        ErrorKind,
        Option<Box<dyn std::error::Error + 'static>>,
        Option<&'static str>,
    );

    impl Error {
        pub fn kind(&self) -> &ErrorKind {
            &self.0
        }
    }

    impl From<ErrorKind> for Error {
        fn from(e: ErrorKind) -> Self {
            Error(e, None, None)
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            self.1.as_ref().map(|e| e.as_ref())
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.0, f)
        }
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            if let Some(ref o) = self.2 {
                std::fmt::Display::fmt(o, f)?;
            }

            std::fmt::Debug::fmt(&self.0, f)?;
            if let Some(e) = self.source() {
                std::fmt::Display::fmt("\nCaused by:\n", f)?;
                std::fmt::Debug::fmt(&e, f)?;
            }
            Ok(())
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;

    pub fn func1() -> Result<()> {
        func2().map_err(mcherr!(ErrorKind::Func2))?;
        let filename = String::from("bar.txt");
        do_some_io().map_err(mcherr!(ErrorKind::IO(filename)))?;
        Ok(())
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
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
        let mut s: &Error = &e;
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
    }
    Ok(())
}
