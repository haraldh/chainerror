/*!

~~~bash
$ cargo run -q --example tutorial2
Error: examples/tutorial2.rs:28: "func1 error"
Caused by:
examples/tutorial2.rs:21: "func2 error"
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
    if let Err(e) = do_some_io() {
        Err(cherr!(e, "func2 error"))?;
    }
    Ok(())
}

fn func1() -> Result<(), Box<Error>> {
    if let Err(e) = func2() {
        Err(cherr!(e, "func1 error"))?;
    }
    Ok(())
}

fn main() -> Result<(), Box<Error>> {
    func1()
}
