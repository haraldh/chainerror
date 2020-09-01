[![Crate](https://img.shields.io/crates/v/chainerror.svg)](https://crates.io/crates/chainerror)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/chainerror/)
[![Coverage Status](https://coveralls.io/repos/github/haraldh/chainerror/badge.svg?branch=master)](https://coveralls.io/github/haraldh/chainerror?branch=master)
[![Workflow Status](https://github.com/haraldh/chainerror/workflows/Rust/badge.svg)](https://github.com/haraldh/chainerror/actions?query=workflow%3A%22Rust%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/haraldh/chainerror.svg)](https://isitmaintained.com/project/haraldh/chainerror "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/haraldh/chainerror.svg)](https://isitmaintained.com/project/haraldh/chainerror "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# chainerror

`chainerror` provides an error backtrace without doing a real backtrace, so even after you `strip` your
binaries, you still have the error backtrace.

`chainerror` has no dependencies!

`chainerror` uses `.source()` of `std::error::Error` along with `#[track_caller]` and `Location` to provide a nice debug error backtrace.
It encapsulates all types, which have `Display + Debug` and can store the error cause internally.

Along with the `ChainError<T>` struct, `chainerror` comes with some useful helper macros to save a lot of typing.

Debug information is worth it!

### Features

`display-cause`
: turn on printing a backtrace of the errors in `Display`

## Tutorial

Read the [Tutorial](https://haraldh.github.io/chainerror/tutorial1.html)

## Examples

```console
$ cargo run -q --example example
Main Error Report: func1 error calling func2

Error reported by Func2Error: func2 error: calling func3
The root cause was: std::io::Error: Kind(
    NotFound
)

Debug Error:
examples/example.rs:46:13: func1 error calling func2
Caused by:
examples/example.rs:21:13: Func2Error(func2 error: calling func3)
Caused by:
examples/example.rs:14:18: Error reading 'foo.txt'
Caused by:
Kind(NotFound)

Alternative Debug Error:
ChainError<example::Func1Error> {
    occurrence: Some(
        "examples/example.rs:46:13",
    ),
    kind: func1 error calling func2,
    source: Some(
        ChainError<example::Func2Error> {
            occurrence: Some(
                "examples/example.rs:21:13",
            ),
            kind: Func2Error(func2 error: calling func3),
            source: Some(
                ChainError<alloc::string::String> {
                    occurrence: Some(
                        "examples/example.rs:14:18",
                    ),
                    kind: "Error reading \'foo.txt\'",
                    source: Some(
                        Kind(
                            NotFound,
                        ),
                    ),
                },
            ),
        },
    ),
}
```

```rust
use chainerror::prelude::v1::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func2() -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().context(format!("Error reading '{}'", filename))?;
    Ok(())
}

fn func1() -> Result<(), Box<dyn Error + Send + Sync>> {
    func2().context("func1 error")?;
    Ok(())
}

if let Err(e) = func1() {
    #[cfg(not(windows))]
    assert_eq!(
        format!("\n{:?}\n", e),
        r#"
src/lib.rs:21:13: func1 error
Caused by:
src/lib.rs:16:18: Error reading 'foo.txt'
Caused by:
Kind(NotFound)
"#
    );
}
```


```rust
use chainerror::prelude::v1::*;
use std::error::Error;
use std::io;
use std::result::Result;

fn do_some_io() -> Result<(), Box<dyn Error + Send + Sync>> {
    Err(io::Error::from(io::ErrorKind::NotFound))?;
    Ok(())
}

fn func3() -> Result<(), Box<dyn Error + Send + Sync>> {
    let filename = "foo.txt";
    do_some_io().context(format!("Error reading '{}'", filename))?;
    Ok(())
}

derive_str_context!(Func2Error);

fn func2() -> ChainResult<(), Func2Error> {
    func3().context(Func2Error("func2 error: calling func3".into()))?;
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
    func2().context(Func1Error::Func2)?;
    let filename = String::from("bar.txt");
    do_some_io().context(Func1Error::IO(filename))?;
    Ok(())
}

if let Err(e) = func1() {
    assert!(match e.kind() {
        Func1Error::Func2 => {
            eprintln!("Main Error Report: func1 error calling func2");
            true
        }
        Func1Error::IO(filename) => {
            eprintln!("Main Error Report: func1 error reading '{}'", filename);
            false
        }
    });

    assert!(e.find_chain_cause::<Func2Error>().is_some());

    if let Some(e) = e.find_chain_cause::<Func2Error>() {
        eprintln!("\nError reported by Func2Error: {}", e)
    }

    assert!(e.root_cause().is_some());

    if let Some(e) = e.root_cause() {
        let io_error = e.downcast_ref::<io::Error>().unwrap();
        eprintln!("\nThe root cause was: std::io::Error: {:#?}", io_error);
    }

    #[cfg(not(windows))]
    assert_eq!(
        format!("\n{:?}\n", e),
        r#"
src/lib.rs:48:13: func1 error calling func2
Caused by:
src/lib.rs:23:13: Func2Error(func2 error: calling func3)
Caused by:
src/lib.rs:16:18: Error reading 'foo.txt'
Caused by:
Kind(NotFound)
"#
    );
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
