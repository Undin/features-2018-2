use std::num::ParseIntError;
use std::borrow::Cow;

fn main() {}

fn as_primitive(a: u8) -> i32 {
    a // as i32
}

fn to_string(a: &str) -> String {
    a // .to_string()
}

fn as_str(a: String) {
    let b: &str = a; // .as_str()
}

fn as_mut_str() {
     let b: &mut str = a; // .as_mut_str()
}

fn cow_to_string(a: Cow<str>) -> String {
    a // .to_string()
}

fn cow_to_str(a: Cow<str>) {
    let b: &str = a.borrow(); // .borrow()
}

fn atr_to_byte_slice(a: &str) {
    let b: &[u8] = a; // .as_ref()
}

fn str_to_int(a: &str) {
    let b: i32 = a; // .parse().unwrap();
}

fn str_to_int_with_result(a: &str) {
    let b: Result<i32, ParseIntError> = a; // .parse();
}

fn str_to_int_ret_result(a: &str) -> Result<(), ParseIntError> {
    let b: i32 = a; // .parse()?;
    Ok(())
}

fn refs(a: i32) {
    let b: &i32 = a; // &a
}

fn derefs(a: &i32) {
    let b: i32 = a; // *a
}

fn refs_derefs(a: Box<&Box<i32>>) {
    let b: &&mut &i32 = a; // &&mut &***a
}

fn complex_example(a: String, b: Cow<str>, c: Box<str>) {
    let vec = Vec::new();
    vec.push("");
    vec.push(a); // .as_str()
    vec.push(b); // .borrow()
    vec.push(c); // &*
}

