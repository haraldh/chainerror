use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

pub struct ChainError<T> {
    #[cfg(feature = "fileline")]
    occurrence: Option<(u32, &'static str)>,
    kind: T,
    error_cause: Option<Box<dyn Error + 'static>>,
}

impl<T: 'static + Display + Debug> ChainError<T> {
    #[cfg(feature = "fileline")]
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn Error + 'static>>,
        occurrence: Option<(u32, &'static str)>,
    ) -> Self {
        Self {
            occurrence,
            kind,
            error_cause,
        }
    }

    #[cfg(not(feature = "fileline"))]
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn Error + 'static>>,
        _occurrence: Option<(u32, &'static str)>,
    ) -> Self {
        Self { kind, error_cause }
    }

    pub fn root_cause(&self) -> Option<&(dyn Error + 'static)> {
        let mut cause = self as &(dyn Error + 'static);
        while let Some(c) = cause.source() {
            cause = c;
        }
        Some(cause)
    }

    pub fn find_cause<U: Error + 'static>(&self) -> Option<&(dyn Error + 'static)> {
        let mut cause = self as &(dyn Error + 'static);
        loop {
            if cause.is::<U>() {
                return Some(cause);
            }

            match cause.source() {
                Some(c) => cause = c,
                None => return None,
            }
        }
    }

    pub fn find_kind<U: 'static + Display + Debug>(&self) -> Option<&ChainError<U>> {
        let mut cause = self as &(dyn Error + 'static);
        loop {
            if cause.is::<ChainError<U>>() {
                return cause.downcast_ref::<ChainError<U>>();
            }

            match cause.source() {
                Some(c) => cause = c,
                None => return None,
            }
        }
    }

    pub fn kind<'a>(&'a self) -> &'a T {
        &self.kind
    }
}

pub trait ChainErrorDown {
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool;
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>>;
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>>;
}

use std::any::TypeId;

impl<U: 'static + Display + Debug> ChainErrorDown for ChainError<U> {
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<U>()
    }

    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        if self.is_chain::<T>() {
            unsafe { Some(&*(self as *const dyn Error as *const &ChainError<T>)) }
        } else {
            None
        }
    }

    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        if self.is_chain::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Error as *mut &mut ChainError<T>)) }
        } else {
            None
        }
    }
}

impl ChainErrorDown for dyn Error + 'static {
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<ChainError<T>>()
    }

    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        self.downcast_ref::<ChainError<T>>()
    }

    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        self.downcast_mut::<ChainError<T>>()
    }
}

impl ChainErrorDown for dyn Error + 'static + Send {
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<ChainError<T>>()
    }

    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        self.downcast_ref::<ChainError<T>>()
    }

    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        self.downcast_mut::<ChainError<T>>()
    }
}

impl ChainErrorDown for dyn Error + 'static + Send + Sync {
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<ChainError<T>>()
    }

    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        self.downcast_ref::<ChainError<T>>()
    }

    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        self.downcast_mut::<ChainError<T>>()
    }
}

impl<T: 'static + Display + Debug> Error for ChainError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(ref e) = self.error_cause {
            Some(e.as_ref())
        } else {
            None
        }
    }
}

impl<T: 'static + Display + Debug> Error for &ChainError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(ref e) = self.error_cause {
            Some(e.as_ref())
        } else {
            None
        }
    }
}

impl<T: 'static + Display + Debug> Error for &mut ChainError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(ref e) = self.error_cause {
            Some(e.as_ref())
        } else {
            None
        }
    }
}

impl<T: 'static + Display + Debug> Display for ChainError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.kind)?;

        #[cfg(feature = "display-cause")]
        {
            if let Some(e) = self.source() {
                writeln!(f, "\nCaused by:")?;
                Display::fmt(&e, f)?;
            }
        }
        Ok(())
    }
}

impl<T: 'static + Display + Debug> Debug for ChainError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        #[cfg(feature = "fileline")]
        {
            if let Some(o) = self.occurrence {
                write!(f, "{}:{}: ", o.1, o.0)?;
            }
        }

        Debug::fmt(&self.kind, f)?;

        #[cfg(feature = "debug-cause")]
        {
            if let Some(e) = self.source() {
                writeln!(f, "\nCaused by:")?;
                Debug::fmt(&e, f)?;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! cherr {
    ( $k:expr ) => {
        ChainError::<_>::new($k, None, Some((line!(), file!())))
    };
    ( $e:expr, $k:expr ) => {
        ChainError::<_>::new($k, Some(Box::from($e)), Some((line!(), file!())))
    };
}

#[macro_export]
macro_rules! mstrerr {
    ( $t:ident, $v:expr $(, $more:expr)* ) => {
        |e| cherr!(e, $t (format!($v, $( $more , )* )))
    };
    ( $t:path, $v:expr $(, $more:expr)* ) => {
        |e| cherr!(e, $t (format!($v, $( $more , )* )))
    };
}

#[macro_export]
macro_rules! derive_str_cherr {
    ($e:ident) => {
        struct $e(String);
        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl ::std::fmt::Debug for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!($e), self.0)
            }
        }
    };
}

pub mod prelude {
    pub use super::{cherr, derive_str_cherr, mstrerr, ChainError, ChainErrorDown};
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::Error as IoError;
    use std::io::ErrorKind as IoErrorKind;
    use std::path::Path;

    use crate::prelude::*;

    #[derive(Clone, PartialEq, Debug)]
    enum ParseError {
        InvalidValue(u32),
        InvalidParameter(String),
        NoOpen,
        NoClose,
    }

    impl ::std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match self {
                ParseError::InvalidValue(a) => write!(f, "InvalidValue: {}", a),
                ParseError::InvalidParameter(a) => write!(f, "InvalidParameter: '{}'", a),
                ParseError::NoOpen => write!(f, "No opening '{{' in config file"),
                ParseError::NoClose => write!(f, "No closing '}}' in config file"),
            }
        }
    }

    fn parse_config(c: String) -> Result<(), Box<Error>> {
        if !c.starts_with('{') {
            Err(cherr!(ParseError::NoOpen))?;
        }
        if !c.ends_with('}') {
            Err(cherr!(ParseError::NoClose))?;
        }
        let c = &c[1..(c.len() - 1)];
        let v = c
            .parse::<u32>()
            .map_err(|e| cherr!(e, ParseError::InvalidParameter(c.into())))?;
        if v > 100 {
            Err(cherr!(ParseError::InvalidValue(v)))?;
        }
        Ok(())
    }

    derive_str_cherr!(ConfigFileError);
    derive_str_cherr!(SeriousError);
    derive_str_cherr!(FileError);
    derive_str_cherr!(AppError);

    fn file_reader(_filename: &Path) -> Result<(), Box<Error>> {
        Err(IoError::from(IoErrorKind::NotFound))
            .map_err(mstrerr!(FileError, "File reader error"))?;
        Ok(())
    }

    fn read_config(filename: &Path) -> Result<(), Box<Error>> {
        if filename.eq(Path::new("global.ini")) {
            // assume we got an IO error
            file_reader(filename).map_err(mstrerr!(
                ConfigFileError,
                "Error reading file {:?}",
                filename
            ))?;
        }
        // assume we read some buffer
        if filename.eq(Path::new("local.ini")) {
            let buf = String::from("{1000}");
            parse_config(buf)?;
        }

        if filename.eq(Path::new("user.ini")) {
            let buf = String::from("foo");
            parse_config(buf)?;
        }

        if filename.eq(Path::new("user2.ini")) {
            let buf = String::from("{foo");
            parse_config(buf)?;
        }

        if filename.eq(Path::new("user3.ini")) {
            let buf = String::from("{foo}");
            parse_config(buf)?;
        }

        if filename.eq(Path::new("custom.ini")) {
            let buf = String::from("{10}");
            parse_config(buf)?;
        }

        if filename.eq(Path::new("essential.ini")) {
            Err(cherr!(SeriousError("Something is really wrong".into())))?;
        }

        Ok(())
    }

    fn read_config_pre(p: &str) -> Result<(), Box<Error>> {
        read_config(Path::new(p)).map_err(mstrerr!(AppError, "{}", p))?;
        Ok(())
    }

    #[test]
    fn test_chain_error() {
        for p in &[
            "global.ini",
            "local.ini",
            "user.ini",
            "user2.ini",
            "user3.ini",
            "custom.ini",
            "essential.ini",
        ] {
            if let Err(e) = read_config_pre(p) {
                let app_err = e.downcast_chain_ref::<AppError>().unwrap();

                match p {
                    &"global.ini" => {
                        assert!(app_err.find_kind::<ConfigFileError>().is_some());
                        assert!(app_err.root_cause().unwrap().is::<IoError>());
                    },
                    _ => {}
                }
            }
        }
    }
}
