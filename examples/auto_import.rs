use std::sync::Mutex;

fn main() {
    let counter = Rc::new(Mutex::new(0));
}