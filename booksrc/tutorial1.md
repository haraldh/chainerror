# Simple String Errors

An easy way of doing error handling in rust is by returning `String` as a `Box<std::error::Error>`.

If the rust `main` function returns an `Err()`, this `Err()` will be displayed with `std::fmt::Debug`.

As you can see by running the example (by pressing the "Play" button in upper right of the code block), 
this only 
prints out the last `Error`.

~~~
Error: StringError("func1 error")
~~~

The next chapters of this tutorial show how `chainerror` adds more information
and improves inspecting the sources of an error.

You can also run the tutorial examples in the checked out 
[chainerror git repo](https://github.com/haraldh/chainerror).
~~~
$ cargo run -q --example tutorial1
~~~

~~~rust
{{#include ../examples/tutorial1.rs}}
~~~