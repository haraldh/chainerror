pub trait ChainError: ::std::error::Error + Sized {
    fn root_cause(&self) -> Option<&(dyn::std::error::Error + 'static)>;
    fn find_cause<T: ::std::error::Error + 'static>(
        &self,
    ) -> Option<&(dyn::std::error::Error + 'static)>;
}

#[macro_export]
macro_rules! chain_error {
    ( $t:ident, $v:expr $(, $more:expr)* ) => {
        |e| <$t> :: new(line!(), file!(), format!($v, $( $more , )* ), Some(e.into()))
    };
    ( $t:path, $v:expr $(, $more:expr)* ) => {
        |e| <$t> :: new(line!(), file!(), format!($v, $( $more , )* ), Some(e.into()))
    };
}

#[macro_export]
macro_rules! into_chain_error {
    ( $v:expr $(, $more:expr)* ) => {
        |e| e.into_chain_error(line!(), file!(), Some(format!($v, $( $more , )* )))
    };
    ( $v:expr $(, $more:expr)* ) => {
        |e| e.into_chain_error(line!(), file!(), Some(format!($v, $( $more , )* )))
    };
    ( ) => {
        |e| e.into_chain_error(line!(), file!(), None)
    };
}

#[macro_export]
macro_rules! throw_error {
    ( $t:ident, $v:expr $(, $more:expr)* ) => {
        Err(<$t> :: new(line!(), file!(), format!($v, $( $more , )*), None))
    };
    ( $t:path, $v:expr $(, $more:expr)* ) => {
        Err(<$t> :: new(line!(), file!(), format!($v, $( $more , )*), None))
    };
}

#[macro_export]
macro_rules! new_chain_error {
    ($e:ident) => {
        pub struct $e {
            line: u32,
            filename: &'static str,
            description: String,
            error_cause: Option<Box<dyn::std::error::Error + 'static>>,
        }

        impl $e {
            pub fn new(
                line: u32,
                filename: &'static str,
                description: String,
                error_cause: Option<Box<dyn::std::error::Error + 'static>>,
            ) -> Self {
                $e {
                    line,
                    filename,
                    description,
                    error_cause,
                }
            }
        }

        impl ChainError for $e {
            fn root_cause(&self) -> Option<&(dyn::std::error::Error + 'static)> {
                let mut cause = self as &(dyn::std::error::Error + 'static);
                while let Some(c) = cause.source() {
                    cause = c;
                }
                Some(cause)
            }

            fn find_cause<T: ::std::error::Error + 'static>(
                &self,
            ) -> Option<&(dyn::std::error::Error + 'static)> {
                let mut cause = self as &(dyn::std::error::Error + 'static);
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
                &self.description
            }

            fn source(&self) -> Option<&(dyn::std::error::Error + 'static)> {
                if let Some(ref e) = self.error_cause {
                    Some(e.as_ref())
                } else {
                    None
                }
            }
        }

        impl ::std::fmt::Display for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                writeln!(f, "{}", self.description)?;
                if let Some(e) = self.source() {
                    writeln!(f, "\nCaused by:")?;
                    ::std::fmt::Display::fmt(&e, f)?;
                }
                Ok(())
            }
        }

        impl ::std::fmt::Debug for $e {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                writeln!(f, "\n{}:{}: {}", self.filename, self.line, self.description)?;
                if let Some(e) = self.source() {
                    writeln!(f, "\nCaused by:")?;
                    ::std::fmt::Debug::fmt(&e, f)?;
                }
                Ok(())
            }
        }
    };
}

pub trait FromChainError<T>: Sized {
    fn from_chain_error(_: T, _: u32, _: &'static str, _: Option<String>) -> Self;
}

pub trait IntoChainError<T>: Sized {
    fn into_chain_error(self, line: u32, filename: &'static str, description: Option<String>) -> T;
}

impl<T, U> IntoChainError<U> for T where U: FromChainError<T>
{
    fn into_chain_error(self, line: u32, filename: &'static str, description: Option<String>) -> U {
        U::from_chain_error(self, line, filename, description)
    }
}

pub mod prelude {
    pub use super::{chain_error, new_chain_error, throw_error, ChainError};
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::error::Error;

    new_chain_error!(MyError);
    new_chain_error!(MyMainError);

    fn throw_error() -> Result<(), MyError> {
        let directory = String::from("ldfhgdfkgjdf");
        ::std::fs::remove_dir(&directory).map_err(chain_error!(
            MyError,
            "Could not remove directory '{}'{}",
            &directory,
            "!"
        ))?;
        Ok(())
    }

    #[test]
    fn it_works() -> Result<(), MyMainError> {
        let res = throw_error().map_err(chain_error!(MyMainError, "I has an error."));

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
