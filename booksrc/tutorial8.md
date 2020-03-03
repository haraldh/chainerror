# Finding an Error cause

To distinguish the errors occuring in various places, we can define named string errors with the
"new type" pattern.

~~~rust,ignore
derive_str_cherr!(Func2Error);
derive_str_cherr!(Func1Error);
~~~

Instead of `ChainError<String>` we now have `struct Func1Error(String)` and `ChainError<Func1Error>`.

In the `main` function you can see, how we can match the different errors.

Also see:
~~~rust,ignore
            if let Some(f2err) = f1err.find_chain_cause::<Func2Error>() {
~~~
as a shortcut to
~~~rust,ignore
            if let Some(f2err) = f1err.find_cause::<ChainError<Func2Error>>() {
~~~
hiding the `ChainError<T>` implementation detail.

~~~rust
{{#include ../examples/tutorial8.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~