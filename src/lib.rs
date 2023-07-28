#![doc = include_str!("../README.md")]
#![deny(clippy::all)]
#![allow(clippy::needless_doctest_main)]
#![deny(missing_docs)]

use std::any::TypeId;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::panic::Location;

/// chains an inner error kind `T` with a causing error
pub struct Error<T> {
    occurrence: Option<String>,
    kind: T,
    error_cause: Option<Box<dyn StdError + 'static + Send + Sync>>,
}

/// convenience type alias
pub type Result<O, E> = std::result::Result<O, Error<E>>;

impl<T: 'static + Display + Debug> Error<T> {
    /// Use the `context()` or `map_context()` Result methods instead of calling this directly
    #[inline]
    pub fn new(
        kind: T,
        error_cause: Option<Box<dyn StdError + 'static + Send + Sync>>,
        occurrence: Option<String>,
    ) -> Self {
        Self {
            occurrence,
            kind,
            error_cause,
        }
    }

    /// return the root cause of the error chain, if any exists
    pub fn root_cause(&self) -> Option<&(dyn StdError + 'static)> {
        self.iter().last()
    }

    /// Find the first error cause of type U, if any exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chainerror::Context as _;
    /// use chainerror::ErrorDown as _;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     Err(io::Error::from(io::ErrorKind::NotFound))?;
    ///     Ok(())
    /// }
    ///
    /// chainerror::str_context!(Func2Error);
    ///
    /// fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     let filename = "foo.txt";
    ///     do_some_io().context(Func2Error(format!("Error reading '{}'", filename)))?;
    ///     Ok(())
    /// }
    ///
    /// chainerror::str_context!(Func1Error);
    ///
    /// fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     func2().context(Func1Error::new("func1 error"))?;
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
    pub fn find_cause<U: StdError + 'static>(&self) -> Option<&U> {
        self.iter()
            .filter_map(<dyn StdError>::downcast_ref::<U>)
            .next()
    }

    /// Find the first error cause of type [`Error<U>`](Error), if any exists
    ///
    /// Same as `find_cause`, but hides the [`Error<U>`](Error) implementation internals
    ///
    /// # Examples
    ///
    /// ```rust
    /// # chainerror::str_context!(FooError);
    /// # let err = chainerror::Error::new(String::new(), None, None);
    /// // Instead of writing
    /// err.find_cause::<chainerror::Error<FooError>>();
    ///
    /// // leave out the chainerror::Error<FooError> implementation detail
    /// err.find_chain_cause::<FooError>();
    /// ```
    #[inline]
    pub fn find_chain_cause<U: StdError + 'static>(&self) -> Option<&Error<U>> {
        self.iter()
            .filter_map(<dyn StdError>::downcast_ref::<Error<U>>)
            .next()
    }

    /// Find the first error cause of type [`Error<U>`](Error) or `U`, if any exists and return `U`
    ///
    /// Same as `find_cause` and `find_chain_cause`, but hides the [`Error<U>`](Error) implementation internals
    ///
    /// # Examples
    ///
    /// ```rust
    /// # chainerror::str_context!(FooErrorKind);
    /// # let err = chainerror::Error::new(String::new(), None, None);
    /// // Instead of writing
    /// err.find_cause::<chainerror::Error<FooErrorKind>>();
    /// // and/or
    /// err.find_chain_cause::<FooErrorKind>();
    /// // and/or
    /// err.find_cause::<FooErrorKind>();
    ///
    /// // leave out the chainerror::Error<FooErrorKind> implementation detail
    /// err.find_kind_or_cause::<FooErrorKind>();
    /// ```
    #[inline]
    pub fn find_kind_or_cause<U: StdError + 'static>(&self) -> Option<&U> {
        self.iter()
            .filter_map(|e| {
                e.downcast_ref::<Error<U>>()
                    .map(|e| e.kind())
                    .or_else(|| e.downcast_ref::<U>())
            })
            .next()
    }

    /// Return a reference to T of [`Error<T>`](Error)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chainerror::Context as _;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     Err(io::Error::from(io::ErrorKind::NotFound))?;
    ///     Ok(())
    /// }
    ///
    /// chainerror::str_context!(Func2Error);
    ///
    /// fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     let filename = "foo.txt";
    ///     do_some_io().context(Func2Error(format!("Error reading '{}'", filename)))?;
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
    /// fn func1() -> chainerror::Result<(), Func1ErrorKind> {
    ///     func2().context(Func1ErrorKind::Func2)?;
    ///     do_some_io().context(Func1ErrorKind::IO("bar.txt".into()))?;
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
    pub fn iter(&self) -> impl Iterator<Item = &(dyn StdError + 'static)> {
        ErrorIter {
            current: Some(self),
        }
    }
}

/// Convenience methods for `Result<>` to turn the error into a decorated [`Error`](Error)
pub trait Context<O, E: Into<Box<dyn StdError + 'static + Send + Sync>>> {
    /// Decorate the error with a `kind` of type `T` and the source `Location`
    fn context<T: 'static + Display + Debug>(self, kind: T) -> std::result::Result<O, Error<T>>;

    /// Decorate the error just with the source `Location`
    fn annotate(self) -> std::result::Result<O, Error<AnnotatedError>>;

    /// Decorate the `error` with a `kind` of type `T` produced with a `FnOnce(&error)` and the source `Location`
    fn map_context<T: 'static + Display + Debug, F: FnOnce(&E) -> T>(
        self,
        op: F,
    ) -> std::result::Result<O, Error<T>>;
}

/// Convenience type to just decorate the error with the source `Location`
pub struct AnnotatedError(());

impl Display for AnnotatedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(passed error)")
    }
}

impl Debug for AnnotatedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(passed error)")
    }
}

impl<O, E: Into<Box<dyn StdError + 'static + Send + Sync>>> Context<O, E>
    for std::result::Result<O, E>
{
    #[track_caller]
    #[inline]
    fn context<T: 'static + Display + Debug>(self, kind: T) -> std::result::Result<O, Error<T>> {
        match self {
            Ok(t) => Ok(t),
            Err(error_cause) => Err(Error::new(
                kind,
                Some(error_cause.into()),
                Some(Location::caller().to_string()),
            )),
        }
    }

    #[track_caller]
    #[inline]
    fn annotate(self) -> std::result::Result<O, Error<AnnotatedError>> {
        match self {
            Ok(t) => Ok(t),
            Err(error_cause) => Err(Error::new(
                AnnotatedError(()),
                Some(error_cause.into()),
                Some(Location::caller().to_string()),
            )),
        }
    }

    #[track_caller]
    #[inline]
    fn map_context<T: 'static + Display + Debug, F: FnOnce(&E) -> T>(
        self,
        op: F,
    ) -> std::result::Result<O, Error<T>> {
        match self {
            Ok(t) => Ok(t),
            Err(error_cause) => {
                let kind = op(&error_cause);
                Err(Error::new(
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
    current: Option<&'a (dyn StdError + 'static)>,
}

impl<'a> Iterator for ErrorIter<'a> {
    type Item = &'a (dyn StdError + 'static);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(StdError::source);
        current
    }
}

impl<T: 'static + Display + Debug> std::ops::Deref for Error<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

/// Convenience trait to hide the [`Error<T>`](Error) implementation internals
pub trait ErrorDown {
    /// Test if of type `Error<T>`
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool;
    /// Downcast to a reference of `Error<T>`
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&Error<T>>;
    /// Downcast to a mutable reference of `Error<T>`
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut Error<T>>;
    /// Downcast to T of `Error<T>`
    fn downcast_inner_ref<T: 'static + StdError>(&self) -> Option<&T>;
    /// Downcast to T mutable reference of `Error<T>`
    fn downcast_inner_mut<T: 'static + StdError>(&mut self) -> Option<&mut T>;
}

impl<U: 'static + Display + Debug> ErrorDown for Error<U> {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<U>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&Error<T>> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(*(self as *const dyn StdError as *const &Error<T>))
            }
        } else {
            None
        }
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut Error<T>> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(&mut *(self as *mut dyn StdError as *mut &mut Error<T>))
            }
        } else {
            None
        }
    }
    #[inline]
    fn downcast_inner_ref<T: 'static + StdError>(&self) -> Option<&T> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(&(*(self as *const dyn StdError as *const &Error<T>)).kind)
            }
        } else {
            None
        }
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + StdError>(&mut self) -> Option<&mut T> {
        if self.is_chain::<T>() {
            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                #[allow(trivial_casts)]
                Some(&mut (*(self as *mut dyn StdError as *mut &mut Error<T>)).kind)
            }
        } else {
            None
        }
    }
}

impl ErrorDown for dyn StdError + 'static {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<Error<T>>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&Error<T>> {
        self.downcast_ref::<Error<T>>()
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut Error<T>> {
        self.downcast_mut::<Error<T>>()
    }

    #[inline]
    fn downcast_inner_ref<T: 'static + StdError>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
            .or_else(|| self.downcast_ref::<Error<T>>().map(|e| e.kind()))
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + StdError>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            return self.downcast_mut::<T>();
        }

        self.downcast_mut::<Error<T>>()
            .and_then(|e| e.downcast_inner_mut::<T>())
    }
}

impl ErrorDown for dyn StdError + 'static + Send {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<Error<T>>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&Error<T>> {
        self.downcast_ref::<Error<T>>()
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut Error<T>> {
        self.downcast_mut::<Error<T>>()
    }

    #[inline]
    fn downcast_inner_ref<T: 'static + StdError>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
            .or_else(|| self.downcast_ref::<Error<T>>().map(|e| e.kind()))
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + StdError>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            return self.downcast_mut::<T>();
        }

        self.downcast_mut::<Error<T>>()
            .and_then(|e| e.downcast_inner_mut::<T>())
    }
}

impl ErrorDown for dyn StdError + 'static + Send + Sync {
    #[inline]
    fn is_chain<T: 'static + Display + Debug>(&self) -> bool {
        self.is::<Error<T>>()
    }

    #[inline]
    fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&Error<T>> {
        self.downcast_ref::<Error<T>>()
    }

    #[inline]
    fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut Error<T>> {
        self.downcast_mut::<Error<T>>()
    }

    #[inline]
    fn downcast_inner_ref<T: 'static + StdError>(&self) -> Option<&T> {
        self.downcast_ref::<T>()
            .or_else(|| self.downcast_ref::<Error<T>>().map(|e| e.kind()))
    }

    #[inline]
    fn downcast_inner_mut<T: 'static + StdError>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            return self.downcast_mut::<T>();
        }

        self.downcast_mut::<Error<T>>()
            .and_then(|e| e.downcast_inner_mut::<T>())
    }
}

impl<T: 'static + Display + Debug> StdError for Error<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.error_cause
            .as_ref()
            .map(|e| e.as_ref() as &(dyn StdError + 'static))
    }
}

impl<T: 'static + Display + Debug> StdError for &mut Error<T> {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.error_cause
            .as_ref()
            .map(|e| e.as_ref() as &(dyn StdError + 'static))
    }
}

impl<T: 'static + Display + Debug> Display for Error<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)?;

        if f.alternate() {
            if let Some(e) = self.source() {
                write!(f, "\nCaused by:\n  {:#}", &e)?;
            }
        }

        Ok(())
    }
}

impl<T: 'static + Display + Debug> Debug for Error<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let mut f = f.debug_struct(&format!("Error<{}>", std::any::type_name::<T>()));

            let f = f
                .field("occurrence", &self.occurrence)
                .field("kind", &self.kind)
                .field("source", &self.source());

            f.finish()
        } else {
            if let Some(ref o) = self.occurrence {
                write!(f, "{}: ", o)?;
            }

            if TypeId::of::<String>() == TypeId::of::<T>()
                || TypeId::of::<&str>() == TypeId::of::<T>()
            {
                Display::fmt(&self.kind, f)?;
            } else {
                Debug::fmt(&self.kind, f)?;
            }

            if let Some(e) = self.source() {
                write!(f, "\nCaused by:\n{:?}", &e)?;
            }
            Ok(())
        }
    }
}

impl<T> From<T> for Error<T>
where
    T: 'static + Display + Debug,
{
    #[track_caller]
    #[inline]
    fn from(e: T) -> Error<T> {
        Error::new(e, None, Some(Location::caller().to_string()))
    }
}
/// Convenience macro to create a "new type" T(String) and implement Display + Debug for T
///
/// # Examples
///
/// ```rust
/// # use chainerror::Context as _;
/// # use chainerror::ErrorDown as _;
/// # use std::error::Error;
/// # use std::io;
/// # use std::result::Result;
/// # fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
/// #     Err(io::Error::from(io::ErrorKind::NotFound))?;
/// #     Ok(())
/// # }
/// chainerror::str_context!(Func2Error);
///
/// fn func2() -> chainerror::Result<(), Func2Error> {
///     let filename = "foo.txt";
///     do_some_io().context(Func2Error(format!("Error reading '{}'", filename)))?;
///     Ok(())
/// }
///
/// chainerror::str_context!(Func1Error);
///
/// fn func1() -> Result<(), Box<dyn Error>> {
///     func2().context(Func1Error::new("func1 error"))?;
///     Ok(())
/// }
/// #     if let Err(e) = func1() {
/// #         if let Some(f1err) = e.downcast_chain_ref::<Func1Error>() {
/// #             assert!(f1err.find_cause::<chainerror::Error<Func2Error>>().is_some());
/// #             assert!(f1err.find_chain_cause::<Func2Error>().is_some());
/// #         } else {
/// #             panic!();
/// #         }
/// #     } else {
/// #         unreachable!();
/// #     }
/// ```
#[macro_export]
macro_rules! str_context {
    ($e:ident) => {
        #[derive(Clone)]
        pub struct $e(pub String);
        impl $e {
            pub fn new<S: Into<String>>(s: S) -> Self {
                $e(s.into())
            }
        }
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

/// Derive an Error for an ErrorKind, which wraps a [`Error`](Error) and implements a `kind()` method
///
/// It basically hides [`Error`](Error) to the outside and only exposes the [`kind()`](Error::kind)
/// method.
///
/// Error::kind() returns the ErrorKind
/// Error::source() returns the parent error
///
/// # Examples
///
/// ```rust
/// use chainerror::Context as _;
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
/// chainerror::err_kind!(Error, ErrorKind);
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
///     do_some_io(filename).map_context(|e| ErrorKind::from_io_error(e, filename.into()))?;
///     do_some_io(filename).map_context(|e| ErrorKind::IO(filename.into()))?;
///     do_some_io(filename).map_context(|e| ErrorKind::from(e))?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! err_kind {
    ($e:ident, $k:ident) => {
        pub struct $e($crate::Error<$k>);

        impl $e {
            pub fn kind(&self) -> &$k {
                self.0.kind()
            }
        }

        impl From<$k> for $e {
            fn from(e: $k) -> Self {
                $e($crate::Error::new(e, None, None))
            }
        }

        impl From<$crate::Error<$k>> for $e {
            fn from(e: $crate::Error<$k>) -> Self {
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
