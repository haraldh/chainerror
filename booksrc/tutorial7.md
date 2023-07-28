# The root cause of all Errors

`chainerror` also has some helper methods:

~~~rust,ignore
fn is_chain<T: 'static + Display + Debug>(&self) -> bool
fn downcast_chain_ref<T: 'static + Display + Debug>(&self) -> Option<&chainerror::Error<T>>
fn downcast_chain_mut<T: 'static + Display + Debug>(&mut self) -> Option<&mut chainerror::Error<T>>
fn root_cause(&self) -> Option<&(dyn Error + 'static)>
fn find_cause<U: Error + 'static>(&self) -> Option<&U>
fn find_chain_cause<U: Error + 'static>(&self) -> Option<&chainerror::Error<U>>
fn kind<'a>(&'a self) -> &'a T
~~~

Using `downcast_chain_ref::<String>()` gives a `chainerror::Error<String>`, which can be used
to call `.find_cause::<io::Error>()`. 

~~~rust,ignore
        if let Some(s) = e.downcast_chain_ref::<String>() {
            if let Some(ioerror) = s.find_cause::<io::Error>() {
~~~

or to use `.root_cause()`, which of course can be of any type implementing `std::error::Error`.

~~~rust,ignore
            if let Some(e) = s.root_cause() {
~~~

~~~rust
{{#include ../examples/tutorial7.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~