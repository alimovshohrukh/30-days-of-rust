# Day 11: Implementing a Simple Stack Data Structure

## Objective
The goal for Day 11 was to create a stack data structure in Rust. The stack supports basic operations: `push`, `pop`, and `peek`. This implementation also includes additional methods for checking if the stack is empty and for finding its size.

## What I Learned
- How to define and work with custom structs in Rust.
- Implementing generic types in structs and methods to allow flexibility in data type.
- Using vectors as internal storage for custom data structures.
- Managing ownership and borrowing to ensure memory safety in Rust.

## Program Structure
1. **Define the Stack Struct**:
    - The `Stack` struct contains a `Vec<T>` to hold the stack’s elements.
2. **Implement Stack Operations**:
    - `push`: Adds an element to the top of the stack.
    - `pop`: Removes and returns the top element (or `None` if empty).
    - `peek`: Returns a reference to the top element without removing it.
    - `is_empty`: Checks if the stack is empty.
    - `size`: Returns the number of elements in the stack.
3. **Testing the Stack**:
    - In `main`, the program pushes elements onto the stack, accesses the top element, and removes elements to demonstrate the stack's behavior.

## Code Example

```rust
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
