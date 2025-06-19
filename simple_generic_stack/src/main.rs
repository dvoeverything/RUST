// - Implement a simple generic stack data structure

// **Requirements:**
// - Define a generic Stack struct that can store items of any type.

// **Implement the following methods for the Stack**

// - new: Creates a new, empty stack.
// - push: Adds an item to the top of the stack.
// - pop: Removes and returns the item from the top of the stack.
// - peek: Returns a reference to the top item without removing it.
// - is_empty: Checks if the stack is empty.
// - size: Returns the number of items in the stack.
// - clear: Removes all items from the stack.



// **Hints:**
// - Define a generic struct whose name is 'Stack' generic over T
// -  Use a 'Vector' to store the items
#[derive(Default)]
struct StackDS<T>{
    stack: Vec<T>,
}

impl StackDS<T>{
    fn new() -> Self{
        Self{
            stack:Vec::new()
        }
    }

    fn push_item(&mut self, item : T){
        self.stack.push(item);
        println!("item successfully added");
    }
    fn pop_item(&mut self)->Option<T>{
            self.stack.pop()
    }

    fn peek(&self)-> Option<&T>{
        self.stack.last()
    }

    fn stack_empty(&self)->bool{
        self.stack.is_empty()
    }

    fn size(&self){
        self.stack.len()
    }

    fn clear(&mut self){
        self.stack.clear()
    }
}
fn main() {
    let mut s = Stack::new();        // Stack<i32>

    s.push(10);
    s.push(20);
    s.push(30);

    println!("peek -> {:?}", s.peek()); // Some(&30)
    println!("size -> {}", s.size());   // 3

    while let Some(v) = s.pop() {
        println!("popped {v}");
    }

    println!("empty? {}", s.is_empty()); // true

    s.push(42);
    s.clear();
    println!("after clear, empty? {}", s.is_empty()); // true

}
