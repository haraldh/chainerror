/*!

`chainerror` provides an error backtrace without doing a real backtrace, so even after you `strip` your
binaries, you still have the error backtrace.

`chainerror` has no dependencies!

`chainerror` uses `.source()` of `std::error::Error` along with `line()!` and `file()!` to provide a nice debug error backtrace.
It encapsulates all types, which have `Display + Debug` and can store the error cause internally.

Along with the `ChainError<T>` struct, `chainerror` comes with some useful helper macros to save a lot of typing.

# Examples

~~~rust
use chainerror::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<Error>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<Error>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
    Ok(())
}

fn func1() -> Result<(), Box<Error>> {
    func2().map_err(mstrerr!("func1 error"))?;
    Ok(())
}

fn main() {
    if let Err(e) = func1() {
        assert_eq!(
            format!("\n{:?}\n", e), r#"
src/lib.rs:20: func1 error
Caused by:
src/lib.rs:15: Error reading 'foo.txt'
Caused by:
Kind(NotFound)
"#
        );
    }
#    else {
#        unreachable!();
#    }
}
~~~


~~~rust
use chainerror::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<Error>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func3() -> Result<(), Box<Error>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
    Ok(())
}

derive_str_cherr!(Func2Error);

fn func2() -> ChainResult<(), Func2Error> {
    func3().map_err(mstrerr!(Func2Error, "func2 error: calling func3"))?;
    Ok(())
}

enum Func1Error {
    Func2,
    IO(String),
}

impl ::std::fmt::Display for Func1Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Func1Error::Func2 => write!(f, "func1 error calling func2"),
            Func1Error::IO(filename) => write!(f, "Error reading '{}'", filename),
        }
    }
}

impl ::std::fmt::Debug for Func1Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn func1() -> ChainResult<(), Func1Error> {
    func2().map_err(|e| cherr!(e, Func1Error::Func2))?;
    let filename = String::from("bar.txt");
    do_some_io().map_err(|e| cherr!(e, Func1Error::IO(filename)))?;
    Ok(())
}

fn main() {
    if let Err(e) = func1() {
        assert!(
            match e.kind() {
                Func1Error::Func2 => {
                    eprintln!("Main Error Report: func1 error calling func2");
                    true
                }
                Func1Error::IO(filename) => {
                    eprintln!("Main Error Report: func1 error reading '{}'", filename);
                    false
                }
            }
        );

        assert!(e.find_chain_cause::<Func2Error>().is_some());

        if let Some(e) = e.find_chain_cause::<Func2Error>() {
            eprintln!("\nError reported by Func2Error: {}", e)
        }


        assert!(e.root_cause().is_some());

        if let Some(e) = e.root_cause() {
            let ioerror = e.downcast_ref::<io::Error>().unwrap();
            eprintln!("\nThe root cause was: std::io::Error: {:#?}", ioerror);
        }

        assert_eq!(
            format!("\n{:?}\n", e), r#"
src/lib.rs:47: func1 error calling func2
Caused by:
src/lib.rs:22: Func2Error(func2 error: calling func3)
Caused by:
src/lib.rs:15: Error reading 'foo.txt'
Caused by:
Kind(NotFound)
"#
        );
    }
#    else {
#        unreachable!();
#    }
}
~~~

!*/

use std::any::TypeId;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

/** chains an inner error kind `T` with a causing error
**/
pub struct ChainError<T> {
    #[cfg(not(feature = "no-fileline"))]
    occurrence: Option<(u32, &'static str)>,
    kind: T,
    error_cause: Option<Box<dyn Error + 'static>>,
}

/// convenience type alias
pub type ChainResult<O, E> = std::result::Result<O, ChainError<E>>;

impl<T: 'static + Display + Debug> ChainError<T> {
    #[cfg(not(feature = "no-fileline"))]
    /// Use the `cherr!()` or `mstrerr!()` macro instead of calling this directly
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

    #[cfg(feature = "no-fileline")]
    /// Use the `cherr!()` or `mstrerr!()` macro instead of calling this directly
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn Error + 'static>>,
        _occurrence: Option<(u32, &'static str)>,
    ) -> Self {
        Self { kind, error_cause }
    }

    /// return the root cause of the error chain, if any exists
    pub fn root_cause(&self) -> Option<&(dyn Error + 'static)> {
        let mut cause = self as &(dyn Error + 'static);
        while let Some(c) = cause.source() {
            cause = c;
        }
        Some(cause)
    }

    /** find the first error cause of type U, if any exists

    # Examples

    ~~~rust
    # use crate::chainerror::*;
    # use std::error::Error;
    # use std::io;
    # use std::result::Result;
    #
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

    fn main() {
        if let Err(e) = func1() {
            if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {

                assert!(f1err.find_cause::<io::Error>().is_some());

                assert!(f1err.find_chain_cause::<Func2Error>().is_some());
            }
    #        else {
    #            panic!();
    #        }
        }
    #    else {
    #         unreachable!();
    #    }
    }
    ~~~
    **/
    pub fn find_cause<U: Error + 'static>(&self) -> Option<&U> {
        let mut cause = self as &(dyn Error + 'static);
        loop {
            if cause.is::<U>() {
                return cause.downcast_ref::<U>();
            }

            match cause.source() {
                Some(c) => cause = c,
                None => return None,
            }
        }
    }

    /** find the first error cause of type ChainError<U>, if any exists

    Same as `find_cause`, but hides the `ChainError<U>` implementation internals

    # Examples

    ~~~rust,ignore
    /// Instead of writing
    err.find_cause::<ChainError<FooError>>();

    /// leave out the ChainError<T> implementation detail
    err.find_chain_cause::<FooError>();
    ~~~

    **/
    pub fn find_chain_cause<U: Error + 'static>(&self) -> Option<&ChainError<U>> {
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

    /** return a reference to T of `ChainError<T>`

    # Examples

    ~~~rust
    # use crate::chainerror::*;
    # use std::error::Error;
    # use std::io;
    # use std::result::Result;
    #
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

    #[derive(Debug)]
    enum Func1ErrorKind {
        Func2,
        IO(String),
    }

    // impl ::std::fmt::Display for Func1ErrorKind {…}
    # impl ::std::fmt::Display for Func1ErrorKind {
    #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    #         match self {
    #             Func1ErrorKind::Func2 => write!(f, "func1 error calling func2"),
    #             Func1ErrorKind::IO(filename) => write!(f, "Error reading '{}'", filename),
    #         }
    #     }
    # }

    fn func1() -> ChainResult<(), Func1ErrorKind> {
        func2().map_err(|e| cherr!(e, Func1ErrorKind::Func2))?;
        do_some_io().map_err(|e| cherr!(e, Func1ErrorKind::IO("bar.txt".into())))?;
        Ok(())
    }

    fn main() {
        if let Err(e) = func1() {
            match e.kind() {
                Func1ErrorKind::Func2 => {},
                Func1ErrorKind::IO(filename) => panic!(),
            }
        }
    #    else {
    #         unreachable!();
    #    }
    }
    ~~~

    **/
    pub fn kind<'a>(&'a self) -> &'a T {
        &self.kind
    }
}

/** convenience trait to hide the `ChainError<T>` implementation internals
**/
pub trait ChainErrorDown {
    /** test if of type `ChainError<T>`
     **/
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool;
    /** downcast to a reference of `ChainError<T>`
     **/
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>>;
    /** downcast to a mutable reference of `ChainError<T>`
     **/
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>>;
}

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
        #[cfg(not(feature = "no-fileline"))]
        {
            if let Some(o) = self.occurrence {
                write!(f, "{}:{}: ", o.1, o.0)?;
            }
        }

        if self.is_chain::<String>() {
            Display::fmt(&self.kind, f)?;
        } else {
            Debug::fmt(&self.kind, f)?;
        }

        #[cfg(not(feature = "no-debug-cause"))]
        {
            if let Some(e) = self.source() {
                writeln!(f, "\nCaused by:")?;
                Debug::fmt(&e, f)?;
            }
        }
        Ok(())
    }
}

/** creates a new `ChainError<T>`

# Examples

Create a new ChainError<FooError>, where `FooError` must implement `Display` and `Debug`.
~~~rust
# use chainerror::*;
#
# #[derive(Debug)]
enum FooError {
    Bar,
    Baz(&'static str),
}
#
# impl ::std::fmt::Display for FooError {
#     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
#         match self {
#             FooError::Bar => write!(f, "Bar Error"),
#             FooError::Baz(s) => write!(f, "Baz Error: '{}'", s),
#         }
#     }
# }

// impl ::std::fmt::Display for FooError

fn do_some_stuff() -> bool {
    false
}

fn func() -> ChainResult<(), FooError> {
    if ! do_some_stuff() {
        Err(cherr!(FooError::Baz("Error")))?;
    }
    Ok(())
}
#
# pub fn main() {
#     match func().unwrap_err().kind() {
#         FooError::Baz(s) if s == &"Error" => {},
#         _ => panic!(),
#     }
# }
~~~

Additionally an error cause can be added.

~~~rust
# use chainerror::*;
# use std::io;
# use std::error::Error;
#
# #[derive(Debug)]
# enum FooError {
#     Bar,
#     Baz(&'static str),
# }
#
# impl ::std::fmt::Display for FooError {
#     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
#         match self {
#             FooError::Bar => write!(f, "Bar Error"),
#             FooError::Baz(s) => write!(f, "Baz Error: '{}'", s),
#         }
#     }
# }
#
fn do_some_stuff() -> Result<(), Box<Error>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func() -> ChainResult<(), FooError> {
    do_some_stuff().map_err(
        |e| cherr!(e, FooError::Baz("Error"))
    )?;
    Ok(())
}
#
# pub fn main() {
#     match func().unwrap_err().kind() {
#         FooError::Baz(s) if s == &"Error" => {},
#         _ => panic!(),
#     }
# }
~~~

**/
#[macro_export]
macro_rules! cherr {
    ( $k:expr ) => {
        ChainError::<_>::new($k, None, Some((line!(), file!())))
    };
    ( $e:expr, $k:expr ) => {
        ChainError::<_>::new($k, Some(Box::from($e)), Some((line!(), file!())))
    };
}

/** convenience macro for |e| cherr!(e, format!(…))

# Examples

~~~rust
# use crate::chainerror::*;
# use std::error::Error;
# use std::io;
# use std::result::Result;
#
# fn do_some_io() -> Result<(), Box<Error>> {
#     Err(io::Error::from(io::ErrorKind::NotFound))?;
#     Ok(())
# }
#
fn func2() -> Result<(), Box<Error>> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
    Ok(())
}

fn func1() -> Result<(), Box<Error>> {
    func2().map_err(mstrerr!("func1 error"))?;
    Ok(())
}

# fn main() {
#     if let Err(e) = func1() {
#         assert_eq!(
#             format!("\n{:?}\n", e), r#"
# src/lib.rs:20: func1 error
# Caused by:
# src/lib.rs:15: Error reading 'foo.txt'
# Caused by:
# Kind(NotFound)
# "#
#         );
#     } else {
#         unreachable!();
#     }
# }
~~~

`mstrerr!()` can also be used to map a new `ChainError<T>`, where T was defined with
`derive_str_cherr!(T)`

~~~rust
# use crate::chainerror::*;
# use std::error::Error;
# use std::io;
# use std::result::Result;
#
# fn do_some_io() -> Result<(), Box<Error>> {
#     Err(io::Error::from(io::ErrorKind::NotFound))?;
#     Ok(())
# }
#
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
#
# fn main() {
#     if let Err(e) = func1() {
#         if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
#             assert!(f1err.find_cause::<ChainError<Func2Error>>().is_some());
#             assert!(f1err.find_chain_cause::<Func2Error>().is_some());
#         } else {
#             panic!();
#         }
#     } else {
#         unreachable!();
#     }
# }
~~~
**/
#[macro_export]
macro_rules! mstrerr {
    ( $t:ident, $v:expr $(, $more:expr)* ) => {
        |e| cherr!(e, $t (format!($v, $( $more , )* )))
    };
    ( $t:path, $v:expr $(, $more:expr)* ) => {
        |e| cherr!(e, $t (format!($v, $( $more , )* )))
    };
    ( $v:expr $(, $more:expr)* ) => {
        |e| cherr!(e, format!($v, $( $more , )* ))
    };
}

/** convenience macro to create a "new type" T(String) and implement Display + Debug for T

~~~rust
# use crate::chainerror::*;
# use std::error::Error;
# use std::io;
# use std::result::Result;
#
# fn do_some_io() -> Result<(), Box<Error>> {
#     Err(io::Error::from(io::ErrorKind::NotFound))?;
#     Ok(())
# }
#
derive_str_cherr!(Func2Error);

fn func2() -> ChainResult<(), Func2Error> {
    let filename = "foo.txt";
    do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
    Ok(())
}

derive_str_cherr!(Func1Error);

fn func1() -> Result<(), Box<Error>> {
    func2().map_err(mstrerr!(Func1Error, "func1 error"))?;
    Ok(())
}
#
# fn main() {
#     if let Err(e) = func1() {
#         if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
#             assert!(f1err.find_cause::<ChainError<Func2Error>>().is_some());
#             assert!(f1err.find_chain_cause::<Func2Error>().is_some());
#         } else {
#             panic!();
#         }
#     } else {
#         unreachable!();
#     }
# }
~~~

**/
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
        impl ::std::error::Error for $e {}
    };
}
