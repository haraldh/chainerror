# Mapping Errors

Now let's get more rust idiomatic by using `.map_err()`.

~~~rust
{{#include ../examples/tutorial3.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#rustdoc_include ../src/lib.rs:-1}}
# }
~~~

If you compare the output to the previous example, you will see,
that:

~~~
Error: src/main.rs:19: "func1 error"
~~~

changed to just:

~~~
src/main.rs:16: "func1 error"
~~~

This is, because we caught the error of `func1()` in `main()` and print it out ourselves.

We can now control, whether to output in `Debug` or `Display` mode.
Maybe depending on `--debug` as a CLI argument.