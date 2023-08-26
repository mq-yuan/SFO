/**
 * Your CQueue object will be instantiated and called as such:
 * let obj = CQueue::new();
 * obj.append_tail(value);
 * let ret_2: i32 = obj.delete_head();
 */
fn main() {
    let mut obj = CQueue::new();
    let value = 3;
    obj.append_tail(value);
    let ret_2: i32 = obj.delete_head();
    println!("{}", ret_2);
}

struct CQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        CQueue {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }

    /* quick */
    // fn delete_head(&mut self) -> i32 {
    //     if !self.stack2.is_empty() {
    //         self.stack2.pop().unwrap()
    //     } else {
    //         if self.stack1.is_empty() {
    //             -1
    //         } else {
    //             while !self.stack1.is_empty() {
    //                 self.stack2.push(self.stack1.pop().unwrap());
    //             }
    //             self.stack2.pop().unwrap()
    //         }
    //     }
    // }

    /* slow */
    fn delete_head(&mut self) -> i32 {
        match self.stack2.pop() {
            Some(val) => val,
            None => {
                // self.stack1.reverse();
                // self.stack2.append(&mut self.stack1);
                while !self.stack1.is_empty() {
                    self.stack2.push(self.stack1.pop().unwrap());
                }
                self.stack2.pop().unwrap_or(-1)
            }
        }
    }
}
