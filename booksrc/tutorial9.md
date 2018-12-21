# Selective Error Handling

What about functions returning different Error types?

In this example `func1()` can return either `Func1ErrorFunc2` or `Func1ErrorIO`.

We might want to `match` on `func1()` with something like:

~~~rust,ignore
fn main() -> Result<(), Box<Error>> {
    match func1() {
        Err(e) if let Some(s) = e.downcast_chain_ref::<Func1ErrorIO>() =>
        eprintln!("Func1ErrorIO:\n{:?}", s),

        Err(e) if let Some(s) = e.downcast_chain_ref::<Func1ErrorFunc2>() =>
        eprintln!("Func1ErrorFunc2:\n{:?}", s),
        
        Ok(_) => {}, 
    }
    Ok(())
}
~~~

but this is not valid rust code, so we end up doing it the hard way.
In the next chapter, we will see, how to solve this more elegantly.

~~~rust
use crate::chainerror::*;
{{#include ../examples/tutorial9.rs:2:}}
# #[allow(dead_code)]
# mod chainerror {
{{#includecomment ../src/lib.rs}}
# }
~~~
