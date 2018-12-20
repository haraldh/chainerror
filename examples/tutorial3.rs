/*!

~~~bash
$ cargo run -q --example tutorial3
examples/tutorial3.rs:30: "func1 error"
Caused by:
examples/tutorial3.rs:25: "func2 error"
Caused by:
StringError("do_some_io error")
~~~

!*/

use chainerror::prelude::*;
use std::error::Error;
use std::result::Result;

fn do_some_io() -> Result<(), Box<Error>> {
    Err("do_some_io error")?;
    Ok(())
}

fn func2() -> Result<(), Box<Error>> {
    do_some_io().map_err(|e| cherr!(e, "func2 error"))?;
    Ok(())
}

fn func1() -> Result<(), Box<Error>> {
    func2().map_err(|e| cherr!(e, "func1 error"))?;
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    if let Err(e) = func1() {
        eprintln!("{:?}", e);
    }
    Ok(())
}
