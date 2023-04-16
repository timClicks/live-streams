use std::ptr;

#[derive(Debug, PartialEq)]
pub struct Node<T: PartialEq> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T: PartialEq> Node<T> {
    fn new(value: T) -> *mut Self {
        Box::into_raw(Box::new(Node {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T: PartialEq> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T: PartialEq> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        unsafe {
            if self.tail.is_null() {
                self.head = new_node;
            } else {
                (*self.tail).next = new_node;
                (*new_node).prev = self.tail;
            }
            self.tail = new_node;
        }
    }

    pub fn remove(&mut self, target: *mut Node<T>) {
        unsafe {
            if (*target).prev.is_null() {
                self.head = (*target).next;
            } else {
                (*(*target).prev).next = (*target).next;
            }

            if !(*target).next.is_null() {
                self.tail = (*target).prev;
            } else {
                (*(*target).next).prev = (*target).prev;
            }

            drop(Box::from_raw(target));
        }
    }

    pub fn find(&self, value: T) -> Option<*mut Node<T>> {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                if (*current).value == value {
                    return Some(current);
                }
                current = (*current).next;
            }
        }
        None
    }

    pub fn iter(&self) -> DoublyLinkedListIterator<T> {
        DoublyLinkedListIterator { current: self.head }
    }
}

pub struct DoublyLinkedListIterator<T: PartialEq> {
    current: *mut Node<T>,
}

impl<T: PartialEq> Iterator for DoublyLinkedListIterator<T> {
    type Item = *const T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        let current_node = self.current;
        let value = unsafe {
            self.current = (*self.current).next;
            &(*current_node).value as *const _
        };
        Some(value)
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
        unsafe {
            println!("Node: {:?}", *node);
        }
    }

    // Find a specific element in the list
    let search_value = 2;
    match list.find(search_value) {
        Some(node) => unsafe {
            println!("Found node with value {}: {:?}", search_value, *node);
        },
        None => println!("Node with value {} not found.", search_value),
    }
}
