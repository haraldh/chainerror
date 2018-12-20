# chainerror

`chainerror` provides an error backtrace like `failure` without doing a real backtrace, so even after you `strip` your
binaries, you still have the error backtrace.

`chainerror` uses `.source()` of `std::error::Error` along with `line()!` and `file()!` to provide a nice debug error backtrace.
It encapsulates all types, which have `Display + Debug` and can store the error cause internally.

Along with the `ChainError<T>` struct, `chainerror` comes with some useful helper macros to save a lot of typing.

Debug information is worth it!

For an introduction read the [Tutorial](https://haraldh.github.io/chainerror/)
