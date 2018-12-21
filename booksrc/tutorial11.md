# Debug for the ErrorKind

One small improvement at the end of the tutorial is to fix the debug output of
`Func1ErrorKind`. As you probably noticed, the output doesn't say much of the enum.

~~~
Debug Error:
src/main.rs:35: Func2
[…]
~~~

As a lazy shortcut, we implement `Debug` by calling `Display` and end up with

~~~
Debug Error:
src/main.rs:40: func1 error calling func2
[…}
~~~

which gives us a lot more detail.

~~~rust
use crate::chainerror::*;
{{#include ../examples/tutorial11.rs:2:}}
# #[allow(dead_code)]
# mod chainerror {
{{#includecomment ../src/lib.rs}}
# }
~~~