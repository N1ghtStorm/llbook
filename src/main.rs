mod my_ll;

use my_ll::*;

fn main() {
    // println!("Hello, world!");
    let mut my_ll = MyLL::new();
    my_ll.add_head(1);
    my_ll.add_head(2);
    my_ll.add_head(3);
    my_ll.add_head(4);
    my_ll.add_head(5);
    my_ll.add_head(6);

    let mut ptr = my_ll.head; 
    unsafe {
        while ptr != 0 as *mut MyLLNode<i32> {
            print!("({})->", (*ptr).val);
            ptr = (*ptr).next;
        }
        println!("null");
    }

}

