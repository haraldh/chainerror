pub trait ChainError: ::std::error::Error + Sized {
    fn new(
        line: u32,
        filename: &'static str,
        description: Option<String>,
        error_cause: Option<Box<dyn std::error::Error + 'static>>,
    ) -> Self;

    fn root_cause(&self) -> Option<&(dyn std::error::Error + 'static)>;
    fn find_cause<T: ::std::error::Error + 'static>(
        &self,
    ) -> Option<&(dyn std::error::Error + 'static)>;
}

pub trait ChainErrorFrom<T>: ChainError {
    fn chain_error_from(_: T, _: u32, _: &'static str, _: Option<String>) -> Self;
}

pub trait IntoChainError<T: ChainError>: Sized {
    fn into_chain_error(self, line: u32, filename: &'static str, description: Option<String>) -> T;
}

impl<T, U> IntoChainError<U> for T
where
    U: ChainErrorFrom<T> + ChainError,
{
    fn into_chain_error(self, line: u32, filename: &'static str, description: Option<String>) -> U {
        U::chain_error_from(self, line, filename, description)
    }
}

#[macro_export]
macro_rules! chain_error_fn {
    ( $t:ident, $v:expr $(, $more:expr)* ) => {
        |e| <$t> :: new(line!(), file!(), Some(format!($v, $( $more , )* )), Some(e.into()))
    };
    ( $t:path, $v:expr $(, $more:expr)* ) => {
        |e| <$t> :: new(line!(), file!(), Some(format!($v, $( $more , )* )), Some(e.into()))
    };
    ( $t:ident) => {
        |e| <$t> :: new(line!(), file!(), None, Some(e.into()))
    };
    ( $t:path) => {
        |e| <$t> :: new(line!(), file!(), None, Some(e.into()))
    };
}

#[macro_export]
macro_rules! into_chain_error_fn {
    ( $v:expr $(, $more:expr)* ) => {
        |e| e.into_chain_error(line!(), file!(), Some(format!($v, $( $more , )* )))
    };
    ( ) => {
        |e| e.into_chain_error(line!(), file!(), None)
    };
}

#[macro_export]
macro_rules! chain_error_from_fn {
    ( $t:expr, $v:expr $(, $more:expr)* ) => {
        |e| ($t).into().chain_error_from(e, line!(), file!(), Some(format!($v, $( $more , )* )))
    };

    ( $t:expr ) => {
        |e| ($t).into().chain_error_from(e, line!(), file!(), None)
    };
}

#[macro_export]
macro_rules! chain_error {
    ( $t:ident, $v:expr $(, $more:expr)* ) => {
        <$t> :: new(line!(), file!(), Some(format!($v, $( $more , )*)), None)
    };
    ( $t:path, $v:expr $(, $more:expr)* ) => {
        <$t> :: new(line!(), file!(), Some(format!($v, $( $more , )*)), None)
    };
}

#[macro_export]
macro_rules! into_chain_error {
    ( $t:expr, $v:expr $(, $more:expr)* ) => {
        $t . into_chain_error(line!(), file!(), Some(format!($v, $( $more , )*)))
    };
    ( $t:expr ) => {
        $t . into_chain_error(line!(), file!(), None)
    };
}

#[macro_export]
macro_rules! derive_chain_error {
    ($e:ident) => {
        pub struct $e {
            line: u32,
            filename: &'static str,
            description: Option<String>,
            error_cause: Option<Box<dyn std::error::Error + 'static>>,
        }

        impl ChainError for $e {
            fn new(
                line: u32,
                filename: &'static str,
                description: Option<String>,
                error_cause: Option<Box<dyn std::error::Error + 'static>>,
            ) -> Self {
                $e {
                    line,
                    filename,
                    description,
                    error_cause,
                }
            }

            fn root_cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
                let mut cause = self as &(dyn std::error::Error + 'static);
                while let Some(c) = cause.source() {
                    cause = c;
                }
                Some(cause)
            }

            fn find_cause<T: ::std::error::Error + 'static>(
                &self,
            ) -> Option<&(dyn std::error::Error + 'static)> {
                let mut cause = self as &(dyn std::error::Error + 'static);
                loop {
                    if cause.is::<T>() {
                        return Some(cause);
                    }

                    match cause.source() {
                        Some(c) => cause = c,
                        None => return None,
                    }
                }
            }
        }

        impl ::std::error::Error for $e {
            fn description(&self) -> &str {
                if let Some(ref d) = self.description {
                    d.as_ref()
                } else {
                    ""
                }
            }

            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                if let Some(ref e) = self.error_cause {
                    Some(e.as_ref())
                } else {
                    None
                }
            }
        }

        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                writeln!(f, "{}", self.description())?;
                if let Some(e) = self.source() {
                    writeln!(f, "\nCaused by:")?;
                    ::std::fmt::Display::fmt(&e, f)?;
                }
                Ok(())
            }
        }

        impl ::std::fmt::Debug for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                writeln!(
                    f,
                    "\n{}:{}: {}",
                    self.filename,
                    self.line,
                    self.description()
                )?;
                if let Some(e) = self.source() {
                    writeln!(f, "\nCaused by:")?;
                    ::std::fmt::Debug::fmt(&e, f)?;
                }
                Ok(())
            }
        }
    };
}

pub mod prelude {
    pub use super::{
        chain_error, chain_error_fn, chain_error_from_fn, derive_chain_error, into_chain_error,
        into_chain_error_fn, ChainError, ChainErrorFrom, IntoChainError,
    };
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::prelude::*;

    derive_chain_error!(MyError);
    derive_chain_error!(MyMainError);

    fn throw_error() -> Result<(), MyError> {
        let directory = String::from("ldfhgdfkgjdf");
        ::std::fs::remove_dir(&directory).map_err(chain_error_fn!(
            MyError,
            "Could not remove directory '{}'{}",
            &directory,
            "!"
        ))?;
        Ok(())
    }

    #[test]
    fn it_works() -> Result<(), MyMainError> {
        let res = throw_error().map_err(chain_error_fn!(MyMainError, "I has an error."));

        if let Err(my_err) = res {
            if let Some(source) = my_err.source() {
                assert!(source.is::<MyError>());
            }
            println!("\nRoot cause is {:#?}\n", my_err.root_cause());
            assert!(my_err.root_cause().unwrap().is::<::std::io::Error>());
            assert!(my_err.find_cause::<::std::io::Error>().is_some());

            if my_err.find_cause::<::std::io::Error>().is_some() {
                println!("Has cause io::Error");
            }
            if my_err.find_cause::<MyError>().is_some() {
                println!("Has cause MyError");
            }
            println!("-----------");
            println!("Display Error:\n{}", my_err);
            println!("-----------");
            println!("Debug Error:  \n{:#?}", my_err);
            println!("-----------");
        };
        //res?;
        Ok(())
    }
}
