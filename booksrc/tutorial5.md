# The source() of Errors

Sometimes you want to inspect the `source()` of an `Error`.
`chainerror` implements `std::error::Error::source()`, so you can get the cause of an error.

~~~rust
{{#include ../examples/tutorial5.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#includecomment ../src/lib.rs}}
# }
~~~

Note, that because we changed the output of the error in `main()` from 
`Debug` to `Display`, we don't see the error backtrace with filename and line number.

To enable the `Display` backtrace, you have to enable the feature `display-cause` for `chainerror`.
 