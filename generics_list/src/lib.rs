#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
    pub len: usize,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        Self {
            head: None,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
            let node = Node {
                value,
                next: self.head.take().map(|node| Box::new(node)),
            };

            self.head = Some(node);
            self.len += 1;
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|boxed| *boxed);
            self.len -= 1;
        };
    }

    pub fn len(&self) -> usize {
        self.len
    }
}