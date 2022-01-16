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

    pub fn push_head(&mut self, val: T) {
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

    pub fn push_back(&mut self, val: T) {
        if self.head == 0 as *mut MyLLNode<T> {
            self.head = MyLLNode::new(val);
            return;
        }

        let mut ptr = self.head; 

        unsafe {
            while (*ptr).next != 0 as *mut MyLLNode<T> {
                ptr = (*ptr).next;
            }
            (*ptr).next = MyLLNode::new(val);
        }
    }

    fn delete_all(&mut self) {
        let mut ptr = self.head; 
        let layout = Layout::new::<MyLLNode<T>>();
        unsafe {
            while ptr != 0 as *mut MyLLNode<T> {
                let next_ptr = (*ptr).next;
                dealloc(ptr as *mut u8, layout);
                ptr = next_ptr;
            }
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

impl<T> Drop for MyLL<T> {
    fn drop(&mut self) {
        self.delete_all();
    }
}