#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value: value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        self.head = self
            .head
            .take()
            .and_then(|node| node.next.map(|node| *node));
    }

    pub fn len(&self) -> usize {
        let mut count = 0 as usize;
        if let Some(head) = &self.head {
            let mut current = Some(head);
            while let Some(node) = current {
                count += 1;
                current = node.next.as_ref().map(|boxed_node| &**boxed_node);
                if node.next.is_none() {
                    break;
                }
            }
        };
        count
    }
}
