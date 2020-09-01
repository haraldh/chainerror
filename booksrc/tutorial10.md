# ErrorKind to the rescue

To cope with different kind of errors, we introduce the kind of an error `Func1ErrorKind` with an enum.

Because we derive `Debug` and implement `Display` our `Func1ErrorKind` enum, this enum can be used as
a `std::error::Error`.

Only returning `Func1ErrorKind` in `func1()` now let us get rid of `Result<(), Box<Error + Send + Sync>>` and we can
use `ChainResult<(), Func1ErrorKind>`.

In `main` we can now directly use the methods of `ChainError<T>` without downcasting the error first.

Also a nice `match` on `ChainError<T>.kind()` is now possible, which returns `&T`, meaning 
`&Func1ErrorKind` here.

~~~rust
{{#include ../examples/tutorial10.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~
