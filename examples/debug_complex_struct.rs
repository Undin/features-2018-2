struct Node<T> {
    name: &'static str,
    data: T,
    children: Vec<Node<T>>
}

impl<T> Node<T> {
    fn new(name: &'static str, data: T) -> Node<T> {
        Node { name, data, children: vec![] }
    }

    fn insert(&mut self, child: Node<T>) {
        self.children.push(child)
    }

    fn print(&self) {}
}

enum Type {
    Red, Black
}

fn main() {
    let a = Node::new("A", (1, Type::Red));
    let b = Node::new("B", (2, Type::Red));
    let mut c = Node::new("C", (3, Type::Black));

    c.insert(a); // breakpoint
    c.insert(b); // breakpoint
    c.print(); // breakpoint
}
