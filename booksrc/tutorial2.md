## Simple Chained String Errors

Now with the help of the `chainerror` crate, we can have a nicer output.

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
{{#include ../examples/tutorial2.rs:11:13}}
~~~

The macro `cherr!(cause, newerror)` stores `cause` as the source/cause of `newerror` and returns 
`newerror`, along with the filename (`file!()`) and line number (`line!()`).

`Err(e)?` then returns the error `e` applying `e.into()`, so that we
again have a `Err(Box<Error>)` as a result.

The `Debug` implementation of `ChainError<T>` (which is returned by `cherr!()`)
prints the `Debug` of `T` prefixed with the stored filename and line number.

`ChainError<T>` is in our case `ChainError<String>`.