struct Point(i32, i32);

// Convert struct `p` to reference `&p`
fn main() {
    let p = Point(1, 2);
    process_point(p);
}

fn process_point(p: &Point) {
    //some code
}