[![Crate](https://img.shields.io/crates/v/chainerror.svg)](https://crates.io/crates/chainerror)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/chainerror/)
[![Coverage Status](https://codecov.io/gh/haraldh/chainerror/branch/master/graph/badge.svg?token=HGLJFGA11B)](https://codecov.io/gh/haraldh/chainerror)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# chainerror

`chainerror` provides an error backtrace without doing a real backtrace, so even after you `strip` your
binaries, you still have the error backtrace.

Having nested function returning errors, the output doesn't tell where the error originates from.

```rust
use std::path::PathBuf;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;
fn read_config_file(path: PathBuf) -> Result<(), BoxedError> {
    // do stuff, return other errors
    let _buf = std::fs::read_to_string(&path)?;
    // do stuff, return other errors
    Ok(())
}

fn process_config_file() -> Result<(), BoxedError> {
    // do stuff, return other errors
    let _buf = read_config_file("foo.txt".into())?;
    // do stuff, return other errors
    Ok(())
}

fn main() {
    if let Err(e) = process_config_file() {
        eprintln!("Error:\n{:?}", e);
    }
}
```

This gives the output:
```console
Error:
Os { code: 2, kind: NotFound, message: "No such file or directory" }
```
and you have no idea where it comes from.


With `chainerror`, you can supply a context and get a nice error backtrace:

```rust
use chainerror::Context as _;
use std::path::PathBuf;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;
fn read_config_file(path: PathBuf) -> Result<(), BoxedError> {
    // do stuff, return other errors
    let _buf = std::fs::read_to_string(&path).context(format!("Reading file: {:?}", &path))?;
    // do stuff, return other errors
    Ok(())
}

fn process_config_file() -> Result<(), BoxedError> {
    // do stuff, return other errors
    let _buf = read_config_file("foo.txt".into()).context("read the config file")?;
    // do stuff, return other errors
    Ok(())
}

fn main() {
    if let Err(e) = process_config_file() {
        eprintln!("Error:\n{:?}", e);
    }
}
```

with the output:
```console
Error:
examples/simple.rs:14:51: read the config file
Caused by:
examples/simple.rs:7:47: Reading file: "foo.txt"
Caused by:
Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

`chainerror` uses `.source()` of `std::error::Error` along with `#[track_caller]` and `Location` to provide a nice debug error backtrace.
It encapsulates all types, which have `Display + Debug` and can store the error cause internally.

Along with the `Error<T>` struct, `chainerror` comes with some useful helper macros to save a lot of typing.

`chainerror` has no dependencies!

Debug information is worth it!

## Tutorial

Read the [Tutorial](https://haraldh.github.io/chainerror/tutorial1.html)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
