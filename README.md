# chainerror

[![Build Status](https://travis-ci.org/haraldh/chainerror.svg?branch=master)](https://travis-ci.org/haraldh/chainerror)
[![Crate](https://img.shields.io/crates/v/chainerror.svg)](https://crates.io/crates/chainerror)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/chainerror/)

`chainerror` provides an error backtrace like `failure` without doing a real backtrace, so even after you `strip` your
binaries, you still have the error backtrace.

`chainerror` has no dependencies!

`chainerror` uses `.source()` of `std::error::Error` along with `line()!` and `file()!` to provide a nice debug error backtrace.
It encapsulates all types, which have `Display + Debug` and can store the error cause internally.

Along with the `ChainError<T>` struct, `chainerror` comes with some useful helper macros to save a lot of typing.

Debug information is worth it!

Now continue reading the
[Tutorial](https://haraldh.github.io/chainerror/tutorial1.html)

## Example:
Output:

~~~
$ cargo run -q --example example
Main Error Report: func1 error calling func2

Error reported by Func2Error: func2 error: calling func3

The root cause was: std::io::Error: Kind(
    NotFound
)

Debug Error:
examples/example.rs:45: func1 error calling func2
Caused by:
examples/example.rs:20: Func2Error(func2 error: calling func3)
Caused by:
examples/example.rs:13: Error reading 'foo.txt'
Caused by:
Kind(NotFound)
~~~

~~~rust,ignore
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
        match e.kind() {
            Func1Error::Func2 => eprintln!("Main Error Report: func1 error calling func2"),
            Func1Error::IO(filename) => {
                eprintln!("Main Error Report: func1 error reading '{}'", filename)
            }
        }

        if let Some(e) = e.find_chain_cause::<Func2Error>() {
            eprintln!("\nError reported by Func2Error: {}", e)
        }

        if let Some(e) = e.root_cause() {
            let ioerror = e.downcast_ref::<io::Error>().unwrap();
            eprintln!("\nThe root cause was: std::io::Error: {:#?}", ioerror);
        }

        eprintln!("\nDebug Error:\n{:?}", e);
    }
}

~~~

## Features

`no-fileline`
: completely turn off storing filename and line

`display-cause`
: turn on printing a backtrace of the errors in `Display`

`no-debug-cause`
: turn off printing a backtrace of the errors in `Debug`
