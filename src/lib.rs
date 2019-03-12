//! `chainerror` provides an error backtrace without doing a real backtrace, so even after you `strip` your
//! binaries, you still have the error backtrace.
//!
//! `chainerror` has no dependencies!
//!
//! `chainerror` uses `.source()` of `std::error::Error` along with `line()!` and `file()!` to provide a nice debug error backtrace.
//! It encapsulates all types, which have `Display + Debug` and can store the error cause internally.
//!
//! Along with the `ChainError<T>` struct, `chainerror` comes with some useful helper macros to save a lot of typing.
//!
//! ## Features
//!
//! `no-fileline`
//! : completely turn off storing filename and line
//!
//! `display-cause`
//! : turn on printing a backtrace of the errors in `Display`
//!
//! `no-debug-cause`
//! : turn off printing a backtrace of the errors in `Debug`
//!
//!
//! # Tutorial
//!
//! Read the [Tutorial](https://haraldh.github.io/chainerror/tutorial1.html)
//!
//! # Examples
//!
//! ```rust
//! use chainerror::*;
//! use std::error::Error;
//! use std::io;
//! use std::result::Result;
//!
//! fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
//!     Err(io::Error::from(io::ErrorKind::NotFound))?;
//!     Ok(())
//! }
//!
//! fn func2() -> Result<(), Box<Error + Send + Sync>> {
//!     let filename = "foo.txt";
//!     do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
//!     Ok(())
//! }
//!
//! fn func1() -> Result<(), Box<Error + Send + Sync>> {
//!     func2().map_err(mstrerr!("func1 error"))?;
//!     Ok(())
//! }
//!
//! fn main() {
//!     if let Err(e) = func1() {
//!         #[cfg(not(windows))]
//!         assert_eq!(
//!             format!("\n{:?}\n", e), r#"
//! src/lib.rs:20: func1 error
//! Caused by:
//! src/lib.rs:15: Error reading 'foo.txt'
//! Caused by:
//! Kind(NotFound)
//! "#
//!         );
//!     }
//! #    else {
//! #        unreachable!();
//! #    }
//! }
//! ```
//!
//!
//! ```rust
//! use chainerror::*;
//! use std::error::Error;
//! use std::io;
//! use std::result::Result;
//!
//! fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
//!     Err(io::Error::from(io::ErrorKind::NotFound))?;
//!     Ok(())
//! }
//!
//! fn func3() -> Result<(), Box<Error + Send + Sync>> {
//!     let filename = "foo.txt";
//!     do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
//!     Ok(())
//! }
//!
//! derive_str_cherr!(Func2Error);
//!
//! fn func2() -> ChainResult<(), Func2Error> {
//!     func3().map_err(mstrerr!(Func2Error, "func2 error: calling func3"))?;
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
//!     func2().map_err(|e| cherr!(e, Func1Error::Func2))?;
//!     let filename = String::from("bar.txt");
//!     do_some_io().map_err(|e| cherr!(e, Func1Error::IO(filename)))?;
//!     Ok(())
//! }
//!
//! fn main() {
//!     if let Err(e) = func1() {
//!         assert!(
//!             match e.kind() {
//!                 Func1Error::Func2 => {
//!                     eprintln!("Main Error Report: func1 error calling func2");
//!                     true
//!                 }
//!                 Func1Error::IO(filename) => {
//!                     eprintln!("Main Error Report: func1 error reading '{}'", filename);
//!                     false
//!                 }
//!             }
//!         );
//!
//!         assert!(e.find_chain_cause::<Func2Error>().is_some());
//!
//!         if let Some(e) = e.find_chain_cause::<Func2Error>() {
//!             eprintln!("\nError reported by Func2Error: {}", e)
//!         }
//!
//!
//!         assert!(e.root_cause().is_some());
//!
//!         if let Some(e) = e.root_cause() {
//!             let io_error = e.downcast_ref::<io::Error>().unwrap();
//!             eprintln!("\nThe root cause was: std::io::Error: {:#?}", io_error);
//!         }
//!
//!         #[cfg(not(windows))]
//!         assert_eq!(
//!             format!("\n{:?}\n", e), r#"
//! src/lib.rs:47: func1 error calling func2
//! Caused by:
//! src/lib.rs:22: Func2Error(func2 error: calling func3)
//! Caused by:
//! src/lib.rs:15: Error reading 'foo.txt'
//! Caused by:
//! Kind(NotFound)
//! "#
//!         );
//!     }
//! #    else {
//! #        unreachable!();
//! #    }
//! }
//! ```

#![deny(
    warnings,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    keyword_idents,
    macro_use_extern_crate,
    missing_debug_implementations,
    missing_docs,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_labels,
    unused_lifetimes,
    unstable_features,
    unreachable_pub,
    future_incompatible,
    missing_copy_implementations,
    missing_doc_code_examples,
    rust_2018_idioms,
    rust_2018_compatibility
)]

use std::any::TypeId;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

/// chains an inner error kind `T` with a causing error
pub struct ChainError<T> {
    #[cfg(not(feature = "no-fileline"))]
    occurrence: Option<&'static str>,
    kind: T,
    error_cause: Option<Box<dyn Error + 'static + Send + Sync>>,
}

/// convenience type alias
pub type ChainResult<O, E> = std::result::Result<O, ChainError<E>>;

impl<T: 'static + Display + Debug> ChainError<T> {
    #[cfg(not(feature = "no-fileline"))]
    /// Use the `cherr!()` or `mstrerr!()` macro instead of calling this directly
    #[inline]
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn Error + 'static + Send + Sync>>,
        occurrence: Option<&'static str>,
    ) -> Self {
        Self {
            occurrence,
            kind,
            error_cause,
        }
    }

    #[cfg(feature = "no-fileline")]
    /// Use the `cherr!()` or `mstrerr!()` macro instead of calling this directly
    #[inline]
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn Error + 'static + Send + Sync>>,
        _occurrence: Option<&'static str>,
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
    /// # use crate::chainerror::*;
    /// # use std::error::Error;
    /// # use std::io;
    /// # use std::result::Result;
    /// fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
    ///     Err(io::Error::from(io::ErrorKind::NotFound))?;
    ///     Ok(())
    /// }
    ///
    /// derive_str_cherr!(Func2Error);
    ///
    /// fn func2() -> Result<(), Box<Error + Send + Sync>> {
    ///     let filename = "foo.txt";
    ///     do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
    ///     Ok(())
    /// }
    ///
    /// derive_str_cherr!(Func1Error);
    ///
    /// fn func1() -> Result<(), Box<Error + Send + Sync>> {
    ///     func2().map_err(mstrerr!(Func1Error, "func1 error"))?;
    ///     Ok(())
    /// }
    ///
    /// fn main() {
    ///     if let Err(e) = func1() {
    ///         if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
    ///
    ///             assert!(f1err.find_cause::<io::Error>().is_some());
    ///
    ///             assert!(f1err.find_chain_cause::<Func2Error>().is_some());
    ///         }
    /// #        else {
    /// #            panic!();
    /// #        }
    ///     }
    /// #    else {
    /// #         unreachable!();
    /// #    }
    /// }
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
    /// # use chainerror::{ChainError, derive_str_cherr};
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
    /// # use chainerror::{ChainError, derive_str_cherr};
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
    /// # use crate::chainerror::*;
    /// # use std::error::Error;
    /// # use std::io;
    /// # use std::result::Result;
    /// fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
    ///     Err(io::Error::from(io::ErrorKind::NotFound))?;
    ///     Ok(())
    /// }
    ///
    /// derive_str_cherr!(Func2Error);
    ///
    /// fn func2() -> Result<(), Box<Error + Send + Sync>> {
    ///     let filename = "foo.txt";
    ///     do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
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
    ///     func2().map_err(|e| cherr!(e, Func1ErrorKind::Func2))?;
    ///     do_some_io().map_err(|e| cherr!(e, Func1ErrorKind::IO("bar.txt".into())))?;
    ///     Ok(())
    /// }
    ///
    /// fn main() {
    ///     if let Err(e) = func1() {
    ///         match e.kind() {
    ///             Func1ErrorKind::Func2 => {}
    ///             Func1ErrorKind::IO(filename) => panic!(),
    ///         }
    ///     }
    /// #    else {
    /// #         unreachable!();
    /// #    }
    /// }
    /// ```
    #[inline]
    pub fn kind(&self) -> &T {
        &self.kind
    }

    /// Returns an Iterator over all error causes/sources
    ///
    /// # Example
    ///
    ///
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &(dyn Error + 'static)> {
        ErrorIter {
            current: Some(self),
        }
    }
}

struct ErrorIter<'a> {
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
}

impl<T: 'static + Display + Debug> Error for ChainError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error_cause.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

impl<T: 'static + Display + Debug> Error for &ChainError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error_cause.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

impl<T: 'static + Display + Debug> Error for &mut ChainError<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error_cause.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
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
        #[cfg(not(feature = "no-fileline"))]
        {
            if let Some(ref o) = self.occurrence {
                Display::fmt(o, f)?;
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

/// `ChainErrorFrom<T>` is similar to `From<T>`
pub trait ChainErrorFrom<T>: Sized {
    /// similar to From<T>::from()
    fn chain_error_from(from: T, line_filename: Option<&'static str>) -> ChainError<Self>;
}

/// `IntoChainError<T>` is similar to `Into<T>`
pub trait IntoChainError<T>: Sized {
    /// similar to Into<T>::into()
    fn into_chain_error(self, line_filename: Option<&'static str>) -> ChainError<T>;
}

impl<T, U> IntoChainError<U> for T
where
    U: ChainErrorFrom<T>,
{
    #[inline]
    fn into_chain_error(self, line_filename: Option<&'static str>) -> ChainError<U> {
        U::chain_error_from(self, line_filename)
    }
}

impl<T, U> ChainErrorFrom<T> for U
where
    T: Into<U>,
    U: 'static + Display + Debug,
{
    #[inline]
    fn chain_error_from(t: T, line_filename: Option<&'static str>) -> ChainError<Self> {
        let e: U = t.into();
        ChainError::<_>::new(e, None, line_filename)
    }
}

/// map into `ChainError<T>` with `IntoChainError`
///
/// adds `line!()` and `file!()` information
#[macro_export]
macro_rules! minto_cherr {
    ( ) => {
        |e| ChainErrorFrom::chain_error_from(e, Some(concat!(file!(), ":", line!(), ": ")))
    };
}

/// into `ChainError<T>` with `IntoChainError`
///
/// adds `line!()` and `file!()` information
#[macro_export]
macro_rules! into_cherr {
    ( $t:expr ) => {
        ChainErrorFrom::chain_error_from($t, Some(concat!(file!(), ":", line!(), ": ")))
    };
}

/// Creates a new `ChainError<T>`
///
/// # Examples
///
/// Create a new ChainError<FooError>, where `FooError` must implement `Display` and `Debug`.
/// ```rust
/// # use chainerror::*;
/// # #[derive(Debug)]
/// enum FooError {
///     Bar,
///     Baz(&'static str),
/// }
/// # impl ::std::fmt::Display for FooError {
/// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
/// #         match self {
/// #             FooError::Bar => write!(f, "Bar Error"),
/// #             FooError::Baz(s) => write!(f, "Baz Error: '{}'", s),
/// #         }
/// #     }
/// # }
///
/// //  impl ::std::fmt::Display for FooError
///
/// fn do_some_stuff() -> bool {
///     false
/// }
///
/// fn func() -> ChainResult<(), FooError> {
///     if ! do_some_stuff() {
///         Err(cherr!(FooError::Baz("Error")))?;
///     }
///     Ok(())
/// }
/// # pub fn main() {
/// #     match func().unwrap_err().kind() {
/// #         FooError::Baz(s) if s == &"Error" => {}
/// #         _ => panic!(),
/// #     }
/// # }
/// ```
///
/// Additionally an error cause can be added.
///
/// ```rust
/// # use chainerror::*;
/// # use std::io;
/// # use std::error::Error;
/// # #[derive(Debug)]
/// # enum FooError {
/// #     Bar,
/// #     Baz(&'static str),
/// # }
/// # impl ::std::fmt::Display for FooError {
/// #     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
/// #         match self {
/// #             FooError::Bar => write!(f, "Bar Error"),
/// #             FooError::Baz(s) => write!(f, "Baz Error: '{}'", s),
/// #         }
/// #     }
/// # }
/// fn do_some_stuff() -> Result<(), Box<Error + Send + Sync>> {
///     Err(io::Error::from(io::ErrorKind::NotFound))?;
///     Ok(())
/// }
///
/// fn func() -> ChainResult<(), FooError> {
///     do_some_stuff().map_err(
///         |e| cherr!(e, FooError::Baz("Error"))
///     )?;
///     Ok(())
/// }
/// # pub fn main() {
/// #     match func().unwrap_err().kind() {
/// #         FooError::Baz(s) if s == &"Error" => {}
/// #         _ => panic!(),
/// #     }
/// # }
/// ```
#[macro_export]
macro_rules! cherr {
    ( $k:expr ) => ({
        ChainError::new($k, None, Some(concat!(file!(), ":", line!(), ": ")))
    });
    ( None, $k:expr ) => ({
        ChainError::new($k, None, Some(concat!(file!(), ":", line!(), ": ")))
    });
    ( None, $fmt:expr, $($arg:tt)+ ) => ({
        cherr!(None, format!($fmt, $($arg)+ ))
    });
    ( None, $fmt:expr, $($arg:tt)+ ) => ({
        cherr!(None, format!($fmt, $($arg)+ ))
    });
    ( $e:path, $k:expr ) => ({
        ChainError::new($k, Some(Box::from($e)), Some(concat!(file!(), ":", line!(), ": ")))
    });
    ( $e:path, $fmt:expr, $($arg:tt)+ ) => ({
        cherr!($e, format!($fmt, $($arg)+ ))
    });

}

/// shortcut for |e| cherr!(e, $k)
#[macro_export]
macro_rules! mcherr {
    ( $k:expr ) => {{
        |e| cherr!(e, $k)
    }};
}

/// Convenience macro for `|e| cherr!(e, format!(…))`
///
/// # Examples
///
/// ```rust
/// # use crate::chainerror::*;
/// # use std::error::Error;
/// # use std::io;
/// # use std::result::Result;
/// # fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
/// #     Err(io::Error::from(io::ErrorKind::NotFound))?;
/// #     Ok(())
/// # }
/// fn func2() -> Result<(), Box<Error + Send + Sync>> {
///     let filename = "foo.txt";
///     do_some_io().map_err(mstrerr!("Error reading '{}'", filename))?;
///     Ok(())
/// }
///
/// fn func1() -> Result<(), Box<Error + Send + Sync>> {
///     func2().map_err(mstrerr!("func1 error"))?;
///     Ok(())
/// }
///
/// # fn main() {
/// #     if let Err(e) = func1() {
/// #         #[cfg(not(windows))]
/// #         assert_eq!(
/// #             format!("\n{:?}\n", e), r#"
/// # src/lib.rs:18: func1 error
/// # Caused by:
/// # src/lib.rs:13: Error reading 'foo.txt'
/// # Caused by:
/// # Kind(NotFound)
/// # "#
/// #         );
/// #     } else {
/// #         unreachable!();
/// #     }
/// # }
/// ```
///
/// `mstrerr!()` can also be used to map a new `ChainError<T>`, where T was defined with
/// `derive_str_cherr!(T)`
///
/// ```rust
/// # use crate::chainerror::*;
/// # use std::error::Error;
/// # use std::io;
/// # use std::result::Result;
/// # fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
/// #     Err(io::Error::from(io::ErrorKind::NotFound))?;
/// #     Ok(())
/// # }
/// derive_str_cherr!(Func2Error);
///
/// fn func2() -> Result<(), Box<Error + Send + Sync>> {
///     let filename = "foo.txt";
///     do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
///     Ok(())
/// }
///
/// derive_str_cherr!(Func1Error);
///
/// fn func1() -> Result<(), Box<Error + Send + Sync>> {
///     func2().map_err(mstrerr!(Func1Error, "func1 error"))?;
///     Ok(())
/// }
/// # fn main() {
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
/// # }
/// ```
#[macro_export]
macro_rules! mstrerr {
    ( $t:path, $msg:expr ) => ({
        |e| cherr!(e, $t ($msg.to_string()))
    });
    ( $t:path, $msg:expr, ) => ({
        |e| cherr!(e, $t ($msg.to_string()))
    });
    ( $t:path, $fmt:expr, $($arg:tt)+ ) => ({
        |e| cherr!(e, $t (format!($fmt, $($arg)+ )))
    });
    ($msg:expr) => ({
        |e| cherr!(e, $msg.to_string())
    });
    ($msg:expr, ) => ({
        |e| cherr!(e, $msg.to_string())
    });
    ($fmt:expr, $($arg:tt)+) => ({
        |e| cherr!(e, format!($fmt, $($arg)+ ))
    });
}

/// Convenience macro for `cherr!(T(format!(…)))` where `T(String)`
///
/// # Examples
///
/// ```rust
/// # use crate::chainerror::*;
/// # use std::error::Error;
/// # use std::result::Result;
/// derive_str_cherr!(Func2Error);
///
/// fn func2() -> ChainResult<(), Func2Error> {
///     let filename = "foo.txt";
///     Err(strerr!(Func2Error, "Error reading '{}'", filename))
/// }
///
/// derive_str_cherr!(Func1Error);
///
/// fn func1() -> Result<(), Box<Error + Send + Sync>> {
///     func2().map_err(mstrerr!(Func1Error, "func1 error"))?;
///     Ok(())
/// }
/// # fn main() {
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
/// # }
/// ```
#[macro_export]
macro_rules! strerr {
    ( $t:path, $msg:expr ) => ({
        cherr!($t ($msg.to_string()))
    });
    ( $t:path, $msg:expr, ) => ({
        cherr!($t ($msg.to_string()))
    });
    ( $t:path, $fmt:expr, $($arg:tt)+ ) => ({
        cherr!($t (format!($fmt, $($arg)+ )))
    });
    ($msg:expr) => ({
        cherr!($msg.to_string())
    });
    ($msg:expr, ) => ({
        cherr!($msg.to_string())
    });
    ($fmt:expr, $($arg:tt)+) => ({
        cherr!(format!($fmt, $($arg)+ ))
    });
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
/// # fn do_some_io() -> Result<(), Box<Error + Send + Sync>> {
/// #     Err(io::Error::from(io::ErrorKind::NotFound))?;
/// #     Ok(())
/// # }
/// derive_str_cherr!(Func2Error);
///
/// fn func2() -> ChainResult<(), Func2Error> {
///     let filename = "foo.txt";
///     do_some_io().map_err(mstrerr!(Func2Error, "Error reading '{}'", filename))?;
///     Ok(())
/// }
///
/// derive_str_cherr!(Func1Error);
///
/// fn func1() -> Result<(), Box<Error + Send + Sync>> {
///     func2().map_err(mstrerr!(Func1Error, "func1 error"))?;
///     Ok(())
/// }
/// # fn main() {
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
/// # }
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

/// Derive an Error, which wraps ChainError and implements a kind() method
///
/// e.kind() returns the kind
#[macro_export]
macro_rules! derive_err_kind {
    ($e:ident, $k:ident) => {
        pub struct $e(ChainError<$k>);

        impl $e {
            pub fn kind(&self) -> &$k {
                self.0.kind()
            }
        }

        impl From<$k> for $e {
            fn from(e: $k) -> Self {
                $e(ChainError::new(e, None, None))
            }
        }

        impl From<ChainError<$k>> for $e {
            fn from(e: ChainError<$k>) -> Self {
                $e(e)
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
