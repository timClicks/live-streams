use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone, Debug)]
pub struct Node<T> {
    value: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Clone, Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_node.clone());
            }
        };
        self.tail = Some(new_node);
    }

    pub fn remove(&mut self, target: Rc<RefCell<Node<T>>>) {
        let prev = target.borrow().prev.clone();
        let next = target.borrow().next.clone();
        match prev {
            Some(ref prev_node) => {
                prev_node.borrow_mut().next = next.clone();
            }
            None => {
                self.head = next.clone();
            }
        }
        match next {
            Some(ref next_node) => {
                next_node.borrow_mut().prev = prev.clone();
            }
            None => {
                self.tail = prev.clone();
            }
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::<i32>::new();
    list.append(1);
    list.append(2);
    // list.append(3);

    println!("Doubly linked list: {:?}", list);
}
