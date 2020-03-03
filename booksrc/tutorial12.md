# Deref for the ErrorKind

Because ChainError<T> implements Deref to &T, we can also match on `*e` instead of `e.kind()`
or call a function with `&e`
~~~rust
{{#include ../examples/tutorial12.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~
