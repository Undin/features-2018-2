fn main() {
    let mut v = Vec::new();
    for x in 2..10000 {
        if is_prime(x) {
            v.push(x);
        }
    }
    println!("v = {:?}", v);
}

fn is_prime(x: i32) -> bool {
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    return true
}
