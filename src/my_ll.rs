use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
pub struct MyLL<T> {
    pub head: *mut MyLLNode<T>
}

impl<T> MyLL<T> {
    pub fn new() -> Self {
        MyLL {
            head: 0 as *mut MyLLNode<T>
        }
    }

    pub fn add_head(&mut self, val: T) {
        if self.head == 0 as *mut MyLLNode<T> {
            self.head = MyLLNode::new(val);
            return;
        }

        let old_head = self.head;
        self.head = MyLLNode::new(val);

        unsafe {
            (*self.head).next = old_head;
        }
        
    }
}

#[derive(Debug)]
pub struct MyLLNode<T> {
    pub val: T,
    pub next: *mut MyLLNode<T>,
}

impl<T> MyLLNode<T> {
    fn new(val: T, ) -> *mut MyLLNode<T> {
        unsafe {
            let layout = Layout::new::<MyLLNode<T>>();
            let ptr = alloc(layout) as *mut MyLLNode<T>;
            *ptr =  MyLLNode {
                val,
                next: 0 as *mut MyLLNode<T>,
            };
            ptr
        }
    }
}