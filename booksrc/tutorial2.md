# Simple Chained String Errors

With relatively small changes and the help of the `cherr!` macro of the `chainerror` crate
the `String` errors are now chained together.

Press the play button in the upper right corner and see the nice debug output.

~~~rust
use crate::chainerror::*;
{{#include ../examples/tutorial2.rs:2:}}
# #[allow(dead_code)]
# mod chainerror {
{{#includecomment ../src/lib.rs}}
# }
~~~

### What did we do here?

~~~rust,ignore
{{#include ../examples/tutorial2.rs:13:15}}
~~~

The macro `cherr!(olderror, newerror)` stores `olderror` as the source/cause of `newerror` 
along with the filename (`file!()`) and line number (`line!()`)
and returns `newerror`.

`Err()?` then returns the inner error applying `.into()`, so that we
again have a `Err(Box<Error>)` as a result.

The `Debug` implementation of `ChainError<T>` (which is returned by `cherr!()`)
prints the `Debug` of `T` prefixed with the stored filename and line number.

`ChainError<T>` in our case is `ChainError<String>`.