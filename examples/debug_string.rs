fn main() {
    let s = concat(&["Hello", ", ", "World!"]);
    println!("{}", s);
}

fn concat(xs: &[&str]) -> String {
    let mut result = String::new();
    for s in xs {
        if s.len() > 1 {
            result.push_str(s);
        }
    }
    return result;
}

