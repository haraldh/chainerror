# More information

To give more context to the error, you want to use `format!`
to extend the information in the context string.

~~~rust
{{#include ../examples/tutorial4.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~
