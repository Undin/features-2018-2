// Rust has different types for strings.
// `String` has ownership over the contents of the string,
// `&str` is just reference to some memory where string located.
//
// There is common case when we have `&str` but we need `String`, i.e. need to allocate new object in heap.
// Two common way to do it:
// 1. use `to_string()` method
// 2. use `String::from()` method

fn main() {
    let s = "Hello, World!";
    consume(s);
}

fn consume(s: String) {
    // come code
}