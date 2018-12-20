use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use std::result::Result as StdResult;

pub struct ChainError<T> {
    #[cfg(not(feature = "no-fileline"))]
    occurrence: Option<(u32, &'static str)>,
    kind: T,
    error_cause: Option<Box<dyn Error + 'static>>,
}

pub type ChainResult<O, E> = StdResult<O, ChainError<E>>;

impl<T: 'static + Display + Debug> ChainError<T> {
    #[cfg(not(feature = "no-fileline"))]
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
        #[cfg(not(feature = "no-fileline"))]
        {
            if let Some(o) = self.occurrence {
                write!(f, "{}:{}: ", o.1, o.0)?;
            }
        }

        Debug::fmt(&self.kind, f)?;

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
    ( $v:expr $(, $more:expr)* ) => {
        |e| cherr!(e, format!($v, $( $more , )* ))
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
        impl ::std::error::Error for $e {}
    };
}

#[macro_export]
macro_rules! try_cherr_ref {
    ( $e:expr, $t:ident ) => {
        $e.downcast_ref::<ChainError<$t>>()
    };
    ( $e:expr, $t:path ) => {
        $e.downcast_ref::<ChainError<$t>>()
    };
}

#[macro_export]
macro_rules! try_cherr_mut {
    ( $e:expr, $t:ident ) => {
        $e.downcast_mut::<ChainError<$t>>()
    };
    ( $e:expr, $t:path ) => {
        $e.downcast_mut::<ChainError<$t>>()
    };
}
