//! `chainerror` provides an error backtrace without doing a real backtrace, so even after you `strip` your
//! binaries, you still have the error backtrace.
//!
//! `chainerror` has no dependencies!
//!
//! `chainerror` uses `.source()` of `std::error::Error` along with `#[track_caller]` and `Location` to provide a nice debug error backtrace.
//! It encapsulates all types, which have `Display + Debug` and can store the error cause internally.
//!
//! Along with the `ChainError<T>` struct, `chainerror` comes with some useful helper macros to save a lot of typing.
//!
//! Debug information is worth it!
//!
//! ## Features
//!
//! `default = [ "location", "debug-cause" ]`
//!
//! `location`
//! : store the error location
//!
//! `display-cause`
//! : turn on printing a backtrace of the errors in `Display`
//!
//! `debug-cause`
//! : print a backtrace of the errors in `Debug`
//!
//!
//! # Tutorial
//!
//! Read the [Tutorial](https://haraldh.github.io/chainerror/tutorial1.html)
//!
//! # Examples
//!
//! ```console
//! $ cargo run -q --example example
//! Main Error Report: func1 error calling func2
//!
//! Error reported by Func2Error: func2 error: calling func3
//! The root cause was: std::io::Error: Kind(
//!     NotFound
//! )
//!
//! Debug Error:
//! examples/example.rs:46:13: func1 error calling func2
//! Caused by:
//! examples/example.rs:21:13: Func2Error(func2 error: calling func3)
//! Caused by:
//! examples/example.rs:14:18: Error reading 'foo.txt'
//! Caused by:
//! Kind(NotFound)
//! Alternative Debug Error:
//! ChainError<example::Func1Error> {
//!     occurrence: Some(
//!         "examples/example.rs:46:13",
//!     ),
//!     kind: func1 error calling func2,
//!     source: Some(
//!         ChainError<example::Func2Error> {
//!             occurrence: Some(
//!                 "examples/example.rs:21:13",
//!             ),
//!             kind: Func2Error(func2 error: calling func3),
//!             source: Some(
//!                 ChainError<alloc::string::String> {
//!                     occurrence: Some(
//!                         "examples/example.rs:14:18",
//!                     ),
//!                     kind: "Error reading \'foo.txt\'",
//!                     source: Some(
//!                         Kind(
//!                             NotFound,
//!                         ),
//!                     ),
//!                 },
//!             ),
//!         },
//!     ),
//! }
//! ```
//!
//! ```rust
//! use chainerror::prelude::v1::*;
//! use std::error::Error;
//! use std::io;
//! use std::result::Result;
//!
//! fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
//!     Err(io::Error::from(io::ErrorKind::NotFound))?;
//!     Ok(())
//! }
//!
//! fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
//!     let filename = "foo.txt";
//!     do_some_io().cherr(format!("Error reading '{}'", filename))?;
//!     Ok(())
//! }
//!
//! fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
//!     func2().cherr("func1 error")?;
//!     Ok(())
//! }
//!
//! if let Err(e) = func1() {
//! #   #[cfg(feature = "debug-cause")]
//!     #[cfg(not(windows))]
//!     assert_eq!(
//!         format!("\n{:?}\n", e),
//!         r#"
//! src/lib.rs:21:13: func1 error
//! Caused by:
//! src/lib.rs:16:18: Error reading 'foo.txt'
//! Caused by:
//! Kind(NotFound)
//! "#
//!     );
//! }
//! #    else {
//! #        unreachable!();
//! #    }
//! ```
//!
//!
//! ```rust
//! use chainerror::prelude::v1::*;
//! use std::error::Error;
//! use std::io;
//! use std::result::Result;
//!
//! fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
//!     Err(io::Error::from(io::ErrorKind::NotFound))?;
//!     Ok(())
//! }
//!
//! fn func3() -> Result<(), Box<dyn Error + Send + Sync>> {
//!     let filename = "foo.txt";
//!     do_some_io().cherr(format!("Error reading '{}'", filename))?;
//!     Ok(())
//! }
//!
//! derive_str_cherr!(Func2Error);
//!
//! fn func2() -> ChainResult<(), Func2Error> {
//!     func3().cherr(Func2Error("func2 error: calling func3".into()))?;
//!     Ok(())
//! }
//!
//! enum Func1Error {
//!     Func2,
//!     IO(String),
//! }
//!
//! impl ::std::fmt::Display for Func1Error {
//!     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//!         match self {
//!             Func1Error::Func2 => write!(f, "func1 error calling func2"),
//!             Func1Error::IO(filename) => write!(f, "Error reading '{}'", filename),
//!         }
//!     }
//! }
//!
//! impl ::std::fmt::Debug for Func1Error {
//!     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//!         write!(f, "{}", self)
//!     }
//! }
//!
//! fn func1() -> ChainResult<(), Func1Error> {
//!     func2().cherr(Func1Error::Func2)?;
//!     let filename = String::from("bar.txt");
//!     do_some_io().cherr(Func1Error::IO(filename))?;
//!     Ok(())
//! }
//!
//! if let Err(e) = func1() {
//!     assert!(match e.kind() {
//!         Func1Error::Func2 => {
//!             eprintln!("Main Error Report: func1 error calling func2");
//!             true
//!         }
//!         Func1Error::IO(filename) => {
//!             eprintln!("Main Error Report: func1 error reading '{}'", filename);
//!             false
//!         }
//!     });
//!
//!     assert!(e.find_chain_cause::<Func2Error>().is_some());
//!
//!     if let Some(e) = e.find_chain_cause::<Func2Error>() {
//!         eprintln!("\nError reported by Func2Error: {}", e)
//!     }
//!
//!     assert!(e.root_cause().is_some());
//!
//!     if let Some(e) = e.root_cause() {
//!         let io_error = e.downcast_ref::<io::Error>().unwrap();
//!         eprintln!("\nThe root cause was: std::io::Error: {:#?}", io_error);
//!     }
//!
//! #   #[cfg(feature = "no-debug-cause")]
//!     #[cfg(not(windows))]
//!     assert_eq!(
//!         format!("\n{:?}\n", e),
//!         r#"
//! src/lib.rs:48:13: func1 error calling func2
//! Caused by:
//! src/lib.rs:23:13: Func2Error(func2 error: calling func3)
//! Caused by:
//! src/lib.rs:16:18: Error reading 'foo.txt'
//! Caused by:
//! Kind(NotFound)
//! "#
//!     );
//! }
//! #    else {
//! #        unreachable!();
//! #    }
//! ```

#![deny(clippy::all)]
#![deny(clippy::integer_arithmetic)]
#![deny(missing_docs)]

use std::any::TypeId;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use std::panic::Location;

pub mod prelude {
    //! convenience prelude
    pub mod v1 {
        //! convenience prelude
        pub use crate::ChainErrorDown as _;
        pub use crate::ResultTrait as _;
        pub use crate::{derive_err_kind, derive_str_cherr, ChainError, ChainResult};
    }
}

/// chains an inner error kind `T` with a causing error
pub struct ChainError<T> {
    #[cfg(feature = "location")]
    occurrence: Option<String>,
    kind: T,
    error_cause: Option<Box<dyn Error + 'static + Send + Sync>>,
}

/// convenience type alias
pub type ChainResult<O, E> = std::result::Result<O, ChainError<E>>;

impl<T: 'static + Display + Debug> ChainError<T> {
    #[cfg(feature = "location")]
    /// Use the `cherr()` or `map_cherr()` Result methods instead of calling this directly
    #[inline]
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn Error + 'static + Send + Sync>>,
        occurrence: Option<String>,
    ) -> Self {
        Self {
            occurrence,
            kind,
            error_cause,
        }
    }

    #[cfg(not(feature = "location"))]
    /// Use the `cherr()` or `map_cherr()` Result methods instead of calling this directly
    #[inline]
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn Error + 'static + Send + Sync>>,
        _occurrence: Option<String>,
    ) -> Self {
        Self { kind, error_cause }
    }

    /// return the root cause of the error chain, if any exists
    pub fn root_cause(&self) -> Option<&(dyn Error + 'static)> {
        self.iter().last()
    }

    /// Find the first error cause of type U, if any exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chainerror::prelude::v1::*;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     Err(io::Error::from(io::ErrorKind::NotFound))?;
    ///     Ok(())
    /// }
    ///
    /// derive_str_cherr!(Func2Error);
    ///
    /// fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     let filename = "foo.txt";
    ///     do_some_io().cherr(Func2Error(format!("Error reading '{}'", filename)))?;
    ///     Ok(())
    /// }
    ///
    /// derive_str_cherr!(Func1Error);
    ///
    /// fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     func2().cherr(Func1Error("func1 error".into()))?;
    ///     Ok(())
    /// }
    ///
    /// if let Err(e) = func1() {
    ///     if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
    ///         assert!(f1err.find_cause::<io::Error>().is_some());
    ///
    ///         assert!(f1err.find_chain_cause::<Func2Error>().is_some());
    ///     }
    /// #        else {
    /// #            panic!();
    /// #        }
    /// }
    /// #    else {
    /// #         unreachable!();
    /// #    }
    /// ```
    #[inline]
    pub fn find_cause<U: Error + 'static>(&self) -> Option<&U> {
        self.iter().filter_map(Error::downcast_ref::<U>).next()
    }

    /// Find the first error cause of type `ChainError<U>`, if any exists
    ///
    /// Same as `find_cause`, but hides the `ChainError<U>` implementation internals
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use chainerror::prelude::v1::*;
    /// # derive_str_cherr!(FooError);
    /// # let err = ChainError::new(String::new(), None, None);
    /// // Instead of writing
    /// err.find_cause::<ChainError<FooError>>();
    ///
    /// // leave out the ChainError<FooError> implementation detail
    /// err.find_chain_cause::<FooError>();
    /// ```
    #[inline]
    pub fn find_chain_cause<U: Error + 'static>(&self) -> Option<&ChainError<U>> {
        self.iter()
            .filter_map(Error::downcast_ref::<ChainError<U>>)
            .next()
    }

    /// Find the first error cause of type `ChainError<U>` or `U`, if any exists and return `U`
    ///
    /// Same as `find_cause` and `find_chain_cause`, but hides the `ChainError<U>` implementation internals
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use chainerror::prelude::v1::*;
    /// # derive_str_cherr!(FooErrorKind);
    /// # let err = ChainError::new(String::new(), None, None);
    /// // Instead of writing
    /// err.find_cause::<ChainError<FooErrorKind>>();
    /// // and/or
    /// err.find_chain_cause::<FooErrorKind>();
    /// // and/or
    /// err.find_cause::<FooErrorKind>();
    ///
    /// // leave out the ChainError<FooErrorKind> implementation detail
    /// err.find_kind_or_cause::<FooErrorKind>();
    /// ```
    #[inline]
    pub fn find_kind_or_cause<U: Error + 'static>(&self) -> Option<&U> {
        self.iter()
            .filter_map(|e| {
                e.downcast_ref::<ChainError<U>>()
                    .map(|e| e.kind())
                    .or_else(|| e.downcast_ref::<U>())
            })
            .next()
    }

    /// Return a reference to T of `ChainError<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chainerror::prelude::v1::*;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     Err(io::Error::from(io::ErrorKind::NotFound))?;
    ///     Ok(())
    /// }
    ///
    /// derive_str_cherr!(Func2Error);
    ///
    /// fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     let filename = "foo.txt";
    ///     do_some_io().cherr(Func2Error(format!("Error reading '{}'", filename)))?;
    ///     Ok(())
    /// }
    ///
    /// #[derive(Debug)]
    /// enum Func1ErrorKind {
    ///     Func2,
    ///     IO(String),
    /// }
    ///
    /// /// impl ::std::fmt::Display for Func1ErrorKind {…}
    /// # impl ::std::fmt::Display for Func1ErrorKind {
    /// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    /// #         match self {
    /// #             Func1ErrorKind::Func2 => write!(f, "func1 error calling func2"),
    /// #             Func1ErrorKind::IO(filename) => write!(f, "Error reading '{}'", filename),
    /// #         }
    /// #     }
    /// # }
    ///
    /// fn func1() -> ChainResult<(), Func1ErrorKind> {
    ///     func2().cherr(Func1ErrorKind::Func2)?;
    ///     do_some_io().cherr(Func1ErrorKind::IO("bar.txt".into()))?;
    ///     Ok(())
    /// }
    ///
    /// if let Err(e) = func1() {
    ///     match e.kind() {
    ///         Func1ErrorKind::Func2 => {}
    ///         Func1ErrorKind::IO(filename) => panic!(),
    ///     }
    /// }
    /// #    else {
    /// #         unreachable!();
    /// #    }
    /// ```
    #[inline]
    pub fn kind(&self) -> &T {
        &self.kind
    }

    /// Returns an Iterator over all error causes/sources
    ///
    /// # Example
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &(dyn Error + 'static)> {
        ErrorIter {
            current: Some(self),
        }
    }
}

/// Convenience methods for `Result<>` to turn the error into a decorated ChainError
pub trait ResultTrait<O, E: Into<Box<dyn Error + 'static + Send + Sync>>> {
    /// Decorate the error with a `kind` of type `T` and the source `Location`
    fn cherr<T: 'static + Display + Debug>(self, kind: T) -> std::result::Result<O, ChainError<T>>;

    /// Decorate the error with a `kind` of type `T` produced with a `FnOnce` and the source `Location`
    fn map_cherr<T: 'static + Display + Debug, F: FnOnce(&E) -> T>(
        self,
        op: F,
    ) -> std::result::Result<O, ChainError<T>>;
}

impl<O, E: Into<Box<dyn Error + 'static + Send + Sync>>> ResultTrait<O, E>
    for std::result::Result<O, E>
{
    #[track_caller]
    fn cherr<T: 'static + Display + Debug>(self, kind: T) -> std::result::Result<O, ChainError<T>> {
        match self {
            Ok(t) => Ok(t),
            Err(error_cause) => Err(ChainError::new(
                kind,
                Some(error_cause.into()),
                Some(Location::caller().to_string()),
            )),
        }
    }

    #[track_caller]
    fn map_cherr<T: 'static + Display + Debug, F: FnOnce(&E) -> T>(
        self,
        op: F,
    ) -> std::result::Result<O, ChainError<T>> {
        match self {
            Ok(t) => Ok(t),
            Err(error_cause) => {
                let kind = op(&error_cause);
                Err(ChainError::new(
                    kind,
                    Some(error_cause.into()),
                    Some(Location::caller().to_string()),
                ))
            }
        }
    }
}

/// An iterator over all error causes/sources
pub struct ErrorIter<'a> {
    current: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = &'a (dyn Error + 'static);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(Error::source);
        current
    }
}

impl<T: 'static + Display + Debug> std::ops::Deref for ChainError<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

/// Convenience trait to hide the `ChainError<T>` implementation internals
pub trait ChainErrorDown {
    /// Test if of type `ChainError<T>`
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool;
    /// Downcast to a reference of `ChainError<T>`
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>>;
    /// Downcast to a mutable reference of `ChainError<T>`
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>>;
    /// Downcast to T of `ChainError<T>`
    fn downcast_inner_ref<T: 'static + Error>(&self) -> Option<&T>;
    /// Downcast to T mutable reference of `ChainError<T>`
    fn downcast_inner_mut<T: 'static + Error>(&mut self) -> Option<&mut T>;
}

impl<U: 'static + Display + Debug> ChainErrorDown for ChainError<U> {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<U>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(&*(self as *const dyn Error as *const &ChainError<T>))
            }
        } else {
            None
        }
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(&mut *(self as *mut dyn Error as *mut &mut ChainError<T>))
            }
        } else {
            None
        }
    }
    #[inline]
    fn downcast_inner_ref<T: 'static + Error>(&self) -> Option<&T> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(&(*(self as *const dyn Error as *const &ChainError<T>)).kind)
            }
        } else {
            None
        }
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + Error>(&mut self) -> Option<&mut T> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(&mut (*(self as *mut dyn Error as *mut &mut ChainError<T>)).kind)
            }
        } else {
            None
        }
    }
}

impl ChainErrorDown for dyn Error + 'static {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<ChainError<T>>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        self.downcast_ref::<ChainError<T>>()
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        self.downcast_mut::<ChainError<T>>()
    }

    #[inline]
    fn downcast_inner_ref<T: 'static + Error>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
            .or_else(|| self.downcast_ref::<ChainError<T>>().map(|e| e.kind()))
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + Error>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            return self.downcast_mut::<T>();
        }

        self.downcast_mut::<ChainError<T>>()
            .and_then(|e| e.downcast_inner_mut::<T>())
    }
}

impl ChainErrorDown for dyn Error + 'static + Send {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<ChainError<T>>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        self.downcast_ref::<ChainError<T>>()
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        self.downcast_mut::<ChainError<T>>()
    }

    #[inline]
    fn downcast_inner_ref<T: 'static + Error>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
            .or_else(|| self.downcast_ref::<ChainError<T>>().map(|e| e.kind()))
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + Error>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            return self.downcast_mut::<T>();
        }

        self.downcast_mut::<ChainError<T>>()
            .and_then(|e| e.downcast_inner_mut::<T>())
    }
}

impl ChainErrorDown for dyn Error + 'static + Send + Sync {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<ChainError<T>>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&ChainError<T>> {
        self.downcast_ref::<ChainError<T>>()
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut ChainError<T>> {
        self.downcast_mut::<ChainError<T>>()
    }

    #[inline]
    fn downcast_inner_ref<T: 'static + Error>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
            .or_else(|| self.downcast_ref::<ChainError<T>>().map(|e| e.kind()))
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + Error>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            return self.downcast_mut::<T>();
        }

        self.downcast_mut::<ChainError<T>>()
            .and_then(|e| e.downcast_inner_mut::<T>())
    }
}

impl<T: 'static + Display + Debug> Error for ChainError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error_cause
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

impl<T: 'static + Display + Debug> Error for &ChainError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error_cause
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

impl<T: 'static + Display + Debug> Error for &mut ChainError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error_cause
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

impl<T: 'static + Display + Debug> Display for ChainError<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            let mut f = f.debug_struct(&format!("ChainError<{}>", std::any::type_name::<T>()));

            #[cfg(feature = "location")]
            let f = f.field("occurrence", &self.occurrence);

            let f = f.field("kind", &self.kind);

            #[cfg(feature = "debug-cause")]
            let f = f.field("source", &self.source());

            f.finish()
        } else {
            #[cfg(feature = "location")]
            {
                if let Some(ref o) = self.occurrence {
                    write!(f, "{}: ", o)?;
                }
            }

            if TypeId::of::<String>() == TypeId::of::<T>()
                || TypeId::of::<&str>() == TypeId::of::<T>()
            {
                Display::fmt(&self.kind, f)?;
            } else {
                Debug::fmt(&self.kind, f)?;
            }

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
}

/// `ChainErrorFrom<T>` is similar to `From<T>`
pub trait ChainErrorFrom<T>: Sized {
    /// similar to From<T>::from()
    fn chain_error_from(from: T, line_filename: Option<String>) -> ChainError<Self>;
}

/// `IntoChainError<T>` is similar to `Into<T>`
pub trait IntoChainError<T>: Sized {
    /// similar to Into<T>::into()
    fn into_chain_error(self, line_filename: Option<String>) -> ChainError<T>;
}

impl<T, U> IntoChainError<U> for T
where
    U: ChainErrorFrom<T>,
{
    #[inline]
    fn into_chain_error(self, line_filename: Option<String>) -> ChainError<U> {
        U::chain_error_from(self, line_filename)
    }
}

impl<T, U> ChainErrorFrom<T> for U
where
    T: Into<U>,
    U: 'static + Display + Debug,
{
    #[inline]
    fn chain_error_from(t: T, line_filename: Option<String>) -> ChainError<Self> {
        let e: U = t.into();
        ChainError::new(e, None, line_filename)
    }
}

/// Convenience macro to create a "new type" T(String) and implement Display + Debug for T
///
/// # Examples
///
/// ```rust
/// # use crate::chainerror::*;
/// # use std::error::Error;
/// # use std::io;
/// # use std::result::Result;
/// # fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
/// #     Err(io::Error::from(io::ErrorKind::NotFound))?;
/// #     Ok(())
/// # }
/// derive_str_cherr!(Func2Error);
///
/// fn func2() -> ChainResult<(), Func2Error> {
///     let filename = "foo.txt";
///     do_some_io().cherr(Func2Error(format!("Error reading '{}'", filename)))?;
///     Ok(())
/// }
///
/// derive_str_cherr!(Func1Error);
///
/// fn func1() -> Result<(), Box<dyn Error>> {
///     func2().cherr(Func1Error("func1 error".into()))?;
///     Ok(())
/// }
/// #     if let Err(e) = func1() {
/// #         if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
/// #             assert!(f1err.find_cause::<ChainError<Func2Error>>().is_some());
/// #             assert!(f1err.find_chain_cause::<Func2Error>().is_some());
/// #         } else {
/// #             panic!();
/// #         }
/// #     } else {
/// #         unreachable!();
/// #     }
/// ```
#[macro_export]
macro_rules! derive_str_cherr {
    ($e:ident) => {
        #[derive(Clone)]
        pub struct $e(pub String);
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

/// Derive an Error for an ErrorKind, which wraps a `ChainError` and implements a `kind()` method
///
/// It basically hides `ChainError` to the outside and only exposes the `kind()`
/// method.
///
/// Error::kind() returns the ErrorKind
/// Error::source() returns the parent error
///
/// # Examples
///
/// ```rust
/// use chainerror::prelude::v1::*;
/// use std::io;
///
/// fn do_some_io(_f: &str) -> std::result::Result<(), io::Error> {
///     return Err(io::Error::from(io::ErrorKind::NotFound));
/// }
///
/// #[derive(Debug, Clone)]
/// pub enum ErrorKind {
///     IO(String),
///     FatalError(String),
///     Unknown,
/// }
///
/// derive_err_kind!(Error, ErrorKind);
///
/// impl std::fmt::Display for ErrorKind {
///     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
///         match self {
///             ErrorKind::FatalError(e) => write!(f, "fatal error {}", e),
///             ErrorKind::Unknown => write!(f, "unknown error"),
///             ErrorKind::IO(filename) => write!(f, "Error reading '{}'", filename),
///         }
///     }
/// }
///
/// impl ErrorKind {
///     fn from_io_error(e: &io::Error, f: String) -> Self {
///         match e.kind() {
///             io::ErrorKind::BrokenPipe => panic!("Should not happen"),
///             io::ErrorKind::ConnectionReset => {
///                 ErrorKind::FatalError(format!("While reading `{}`: {}", f, e))
///             }
///             _ => ErrorKind::IO(f),
///         }
///     }
/// }
///
/// impl From<&io::Error> for ErrorKind {
///     fn from(e: &io::Error) -> Self {
///         ErrorKind::IO(format!("{}", e))
///     }
/// }
///
/// pub fn func1() -> std::result::Result<(), Error> {
///     let filename = "bar.txt";
///
///     do_some_io(filename)
///         .map_cherr(|e| ErrorKind::from_io_error(e, filename.into()))?;
///     do_some_io(filename).map_cherr(|e| ErrorKind::IO(filename.into()))?;
///     do_some_io(filename).map_cherr(|e| ErrorKind::from(e))?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! derive_err_kind {
    ($e:ident, $k:ident) => {
        pub struct $e($crate::ChainError<$k>);

        impl $e {
            pub fn kind(&self) -> &$k {
                self.0.kind()
            }
        }

        impl From<$k> for $e {
            fn from(e: $k) -> Self {
                $e($crate::ChainError::new(e, None, None))
            }
        }

        impl From<ChainError<$k>> for $e {
            fn from(e: $crate::ChainError<$k>) -> Self {
                $e(e)
            }
        }

        impl From<&$e> for $k
        where
            $k: Clone,
        {
            fn from(e: &$e) -> Self {
                e.kind().clone()
            }
        }

        impl std::error::Error for $e {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                self.0.source()
            }
        }

        impl std::fmt::Display for $e {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for $e {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }
    };
}
