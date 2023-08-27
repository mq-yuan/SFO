struct MinStack {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
        match self.stack2.last() {
            Some(&m) if x <= m => self.stack2.push(x),
            None => self.stack2.push(x),
            _ => {}
        }
        // if self.stack2.is_empty() {
        //     self.stack2.push(x);
        // } else if x <= self.stack2[self.stack2.len() - 1] {
        //     self.stack2.push(x);
        // }
    }

    fn pop(&mut self) {
        if self.stack1.pop().unwrap_or(i32::MAX) == self.min() {
            self.stack2.pop();
        }
    }

    fn top(&self) -> i32 {
        // self.stack1[self.stack1.len() - 1]
        match self.stack1.last() {
            Some(&x) => x,
            None => i32::MAX,
        }
    }

    fn min(&self) -> i32 {
        match self.stack2.last() {
            Some(&x) => x,
            None => i32::MAX,
        }
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.min();
 */
fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    let res1 = obj.min();
    obj.pop();
    let res2 = obj.top();
    let res3 = obj.min();
    println!("{}, {}, {}", res1, res2, res3);
}
