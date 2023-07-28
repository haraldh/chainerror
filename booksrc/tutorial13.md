# Writing a library

I would advise to only expose an `mycrate::ErrorKind` and type alias `mycrate::Error` to `chainerror::Error<mycrate::ErrorKind>`
so you can tell your library users to use the `.kind()` method as `std::io::Error` does.

If you later decide to make your own `Error` implementation, your library users don't
have to change much or anything.

~~~rust
# #[allow(dead_code)]
# #[macro_use]
# pub mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
pub mod mycrate {
    use crate::chainerror::*; // omit the `crate::` part
{{#include ../examples/tutorial13.rs:3:}}
~~~
