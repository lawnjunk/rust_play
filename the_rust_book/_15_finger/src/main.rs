use std::rc::{Rc,Weak};
use std::cell::RefCell;

fn point_to_a_ting(){
    let x = Box::new(5);
    assert!(*x == 5);
    println!("boxes just point to things: {}", x);
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug)]
struct List {
    head: Option<Rc<RefCell<Node>>>,
}

impl List {
    fn push_front(&mut self, value: i32 ) {
        match self.head.take() {
            None => {
                self.head = Some(Rc::new(RefCell::new(Node {
                    value,
                    next: None,
                    prev: None,
                })))
            }
            Some(head) => {
                let new_head = Rc::new(RefCell::new(Node {
                    value,
                    next: None,
                    prev: None,
                }));
                let old_head = head;
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head)
            }
        };
    }
}

fn point_to_own_thing(){
    let mut a = List {
        head: None,
    };
    a.push_front(3);
    a.push_front(5);
    a.push_front(7);
    a.push_front(9);

    println!("a {:?}", a.head);
}

fn main() {
    point_to_a_ting();
    point_to_own_thing();
}
