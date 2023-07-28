# Finding an Error cause

To distinguish the errors occurring in various places, we can define named string errors with the
"new type" pattern.

~~~rust,ignore
chainerror::str_context!(Func2Error);
chainerror::str_context!(Func1Error);
~~~

Instead of `chainerror::Error<String>` we now have `struct Func1Error(String)` and `chainerror::Error<Func1Error>`.

In the `main` function you can see, how we can match the different errors.

Also see:
~~~rust,ignore
            if let Some(f2err) = f1err.find_chain_cause::<Func2Error>() {
~~~
as a shortcut to
~~~rust,ignore
            if let Some(f2err) = f1err.find_cause::<chainerror::Error<Func2Error>>() {
~~~
hiding the `chainerror::Error<T>` implementation detail.

~~~rust
{{#include ../examples/tutorial8.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~
