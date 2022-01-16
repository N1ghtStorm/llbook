mod my_ll;

use my_ll::*;

fn main() {
    // println!("Hello, world!");
    let mut my_ll = MyLL::new();
    my_ll.push_head(1);
    my_ll.push_head(2);
    my_ll.push_head(3);
    my_ll.push_head(4);
    my_ll.push_head(5);
    my_ll.push_head(6);
    
    my_ll.push_back(1);
    my_ll.push_back(2);
    my_ll.push_back(3);
    my_ll.push_back(4);
    my_ll.push_back(5);
    my_ll.push_back(6);

    loop {
        let mut ptr = my_ll.head; 
        unsafe {
            while ptr != 0 as *mut MyLLNode<i32> {
                print!("({})->", (*ptr).val);
                ptr = (*ptr).next;
            }
            println!("null");
        }
    }


}

