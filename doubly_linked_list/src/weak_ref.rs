use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::iter::Iterator;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    prev: WeakLink<T>,
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

#[derive(Debug)]
pub struct DoublyLinkedList<T: PartialEq> {
    head: Link<T>,
    tail: WeakLink<T>,
}

impl<T: PartialEq> DoublyLinkedList<T> {
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
                old_tail.upgrade().unwrap().borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_node.clone());
            }
        };
        self.tail = Some(Rc::downgrade(&new_node));
    }

    pub fn remove(&mut self, target: &Rc<RefCell<Node<T>>>) {
        let target_borrowed = target.borrow_mut();
        if let Some(prev) = target_borrowed.prev.as_ref().and_then(|w| w.upgrade()) {
            prev.borrow_mut().next = target_borrowed.next.clone();
        } else {
            self.head = target_borrowed.next.clone();
        }

        if let Some(next) = target_borrowed.next.as_ref() {
            next.borrow_mut().prev = target_borrowed.prev.clone();
        } else {
            self.tail = target_borrowed.prev.clone();
        }
    }

    pub fn find(&self, value: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().value == value {
                return Some(node.clone());
            }
            current = node.borrow().next.clone();
        }
        None
    }

    pub fn iter(&self) -> DoublyLinkedListRefIterator<T> {
        DoublyLinkedListRefIterator {
            current: self.head.clone(),
        }
    }
}

pub struct DoublyLinkedListIterator<T: PartialEq> {
    current: Link<T>,
}

impl<T: PartialEq> Iterator for DoublyLinkedListIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.clone() {
            self.current = current.borrow().next.clone();
            Some(current)
        } else {
            None
        }
    }
}

impl<T: PartialEq> IntoIterator for DoublyLinkedList<T> {
    type Item = Rc<RefCell<Node<T>>>;
    type IntoIter = DoublyLinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DoublyLinkedListIterator {
            current: self.head,
        }
    }
}

// fn main() {
//     let mut list = DoublyLinkedList::<i32>::new();

//     // Append some values to the list
//     list

pub struct DoublyLinkedListRefIterator<T: PartialEq> {
    current: Link<T>,
}

impl<T: PartialEq> Iterator for DoublyLinkedListRefIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current.clone() {
            self.current = current.borrow().next.clone();
            Some(current)
        } else {
            None
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::<i32>::new();
    list.append(1);
    list.append(2);
    list.append(3);

    println!("Doubly linked list: {:?}", list);

    // Iterate over the list and print each node
    for node in list.iter() {
        println!("Node: {:?}", *node);
    }

    // Find a specific element in the list
    let search_value = 2;
    match list.find(search_value) {
        Some(node) => {
            println!("Found node with value {}: {:?}", search_value, *node);
        },
        None => println!("Node with value {} not found.", search_value),
    }
}