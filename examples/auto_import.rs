use std::sync::Mutex;

fn main() {
    let counter = Arc::new(Mutex::new(0));
}