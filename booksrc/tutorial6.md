# Downcast the Errors

`std::error::Error` comes with some helper methods to get to the original object of the 
`&(dyn Error + 'static)` returned by `.source()`.

~~~rust,ignore
pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T>
pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T>
~~~

This is how it looks like, when using those:

~~~rust
{{#include ../examples/tutorial6.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#includecomment ../src/lib.rs}}
# }
~~~