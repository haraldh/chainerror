# Saving coding chars

Because decorating an error with more information should not
let you jump through hoops, `chainerror` has a quick macro for that.

`mstrerror!()` fits right into `.map_err()` letting you quickly add
more debug strings.

`mstrerror!()` even understands `format!()` syntax like `println!()`.

~~~rust
{{#include ../examples/tutorial4.rs}}
# #[allow(dead_code)]
# mod chainerror {
{{#includecomment ../src/lib.rs}}
# }
~~~