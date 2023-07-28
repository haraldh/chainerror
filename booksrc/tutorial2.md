# Simple Chained String Errors

With relatively small changes and the help of the `context()` method of the `chainerror` crate
the `&str` errors are now chained together.

Press the play button in the upper right corner and see the nice debug output.

~~~rust
{{#include ../examples/tutorial2.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~

### What did we do here?

~~~rust,ignore
{{#include ../examples/tutorial2.rs:13:15}}
~~~

The function `context(newerror)` stores `olderror` as the source/cause of `newerror` 
along with the `Location` of the `context()` call and returns `Err(newerror)`.

`?` then returns the inner error applying `.into()`, so that we
again have a `Err(Box<Error + Send + Sync>)` as a result.

The `Debug` implementation of `chainerror::Error<T>` (which is returned by `context()`)
prints the `Debug` of `T` prefixed with the stored filename and line number.

`chainerror::Error<T>` in our case is `chainerror::Error<&str>`.
