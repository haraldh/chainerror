# Simple String Errors

The most simplest of doing error handling in rust is by returning `String` as a `Box<Error>`.

As you can see by running the example (the "Play" button in upper right of the code block), this only 
prints out the last `Error`.

If the rust `main` function returns an Err(), this Err() will be displayed with `std::fmt::Debug`.

~~~rust
{{#include ../examples/tutorial1.rs:2:}}
~~~