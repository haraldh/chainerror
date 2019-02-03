# Deref for the ErrorKind

Because ChainError<T> implements Deref to &T, we can also match on `*e` instead of `e.kind()`
or call a function with `&e`
~~~rust
use crate::chainerror::*;
{{#include ../examples/tutorial12.rs:2:}}
# #[allow(dead_code)]
# mod chainerror {
{{#includecomment ../src/lib.rs}}
# }
~~~
