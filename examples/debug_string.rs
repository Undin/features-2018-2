fn main() {
    println!("{:?}", sorted_words("Rust is a systems programming language"));
}

fn sorted_words(xs: &str) -> Vec<String> {
    let mut words = xs.split_whitespace()
        .map(|s| {
            s.to_lowercase() // breakpoint
        })
        .collect::<Vec<String>>();
    words.sort();
    return words; // breakpoint
}
