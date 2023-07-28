# The source() of Errors

Sometimes you want to inspect the `source()` of an `Error`.
`chainerror` implements `std::error::Error::source()`, so you can get the cause of an error.

~~~rust
{{#include ../examples/tutorial5.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~

Note, that because we changed the output of the error in `main()` from 
`Debug` to `Display`, we don't see the error backtrace with filename and line number.

To use the `Display` backtrace, you have to use the alternative display format output `{:#}`.
