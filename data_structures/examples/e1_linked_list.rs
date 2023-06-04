#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut old_head) => {
                self.head = old_head.next.take();
                Some(old_head.data)
            } 
        }
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.push(1);
    list.push(9);

    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
}