struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Top element: {:?}", stack.peek());
    println!("Stack size: {}", stack.size());

    println!("Popping element: {:?}", stack.pop());
    println!("Top element after pop: {:?}", stack.peek());
    println!("Is the stack empty? {}", stack.is_empty());

    stack.pop();
    stack.pop();

    println!("Is the stack empty after popping all elements? {}", stack.is_empty());
}
