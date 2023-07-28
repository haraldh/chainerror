# Mapping Errors

Now let's get more rust idiomatic by using `.context()` directly on the previous `Result`.

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
Error: examples/tutorial2.rs:20:16: func1 error
~~~

changed to just:

~~~
examples/tutorial3.rs:17:13: func1 error
~~~

This is, because we caught the error of `func1()` in `main()` and print it out ourselves.

We can now control, whether to output in `Debug` or `Display` mode.
Maybe depending on `--debug` as a CLI argument.
