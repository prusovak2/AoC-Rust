use std::mem;

// problem of this implemenatation of the linked list is that the first element of the 
// list is allocated on the stack while the rest is on the heap - it is not allocate uniformly
// [] = Stack
// () = Heap
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
// that makes splitting the list less efficient 
#[derive(Debug)]
pub enum ListHeadOnStack<T> {
    Nil,
    Conns(T, Box<ListHeadOnStack<T>>)
}

pub fn example() {
    let list = ListHeadOnStack::Conns(1, Box::new(ListHeadOnStack::Conns(2, Box::new(ListHeadOnStack::Nil))));
    println!("{:?}", list);
}

// to avoid this non-uniform allocation, ee need to better separate out the idea of having an element from allocating another list
// this approach gets rid of non-uniform allocation while still taking advatage of rust null-pointer optimization
// its problem is, however, that we are forced to have a node struct (that is just an implementation detail) public
pub struct PubNode<T> {
    elem: T,
    next: PubNodeList<T>
}

pub enum PubNodeList<T> {
    Empty,
    More(Box<PubNode<T>>)
}

// this takes care of the visibility problem
// we have
// * enum that "represent a ponter" - it is eiher empty (null) or it contains a poinet to the next element
// * node struct that contains element and next "pointer", nothing new here
// * List struct that only contains a head pointer 
pub struct List<T> {
    head: Link<T>
}

struct Node<T> {
    elem: T,
    next: Link<T>
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: Link::Empty }
    }

    // rust won't allow us to move out of head, we need to use std::mem::replace funcion!
    // pub fn push(&mut self, elem: T) {
    //     let new_node = Node {
    //         elem: elem,
    //         next: self.head // error[E0507]: cannot move out of borrowed content
    //     };
    //     self.head = Link::More(Box::new(new_node));
    // }

    // mem::replace head temporarily with an "null pointer" while new_node is being constructed and
    // then set  it to a new node
    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            // set nest to what used to be in head and set head to Empty (temporarily)
            next: std::mem::replace(&mut self.head, Link::Empty) // THIS IS HOW TO DO IT!
        };
        self.head = Link::More(Box::new(new_node));
    }

    // pub fn pop(&mut self) -> Option<T> {
    //     match &mut self.head {
    //         Link::Empty => None,
    //         Link::More(boxed_node) => {
    //             let res = Some(boxed_node.elem);
    //             std::mem::replace(&mut self.head, boxed_node.next);
    //             res
    //         }
    //     }
    // }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {  // match on what was in the head while taking an ownership of it
            Link::Empty => None, // if it was empty, replacing it with empty is fine
            Link::More(boxed_node) => {
                let res = Some(boxed_node.elem); // we can take elem, because we own the node that used to be a head
                self.head = boxed_node.next; // but we need to make sure to set the head to what used to be a second node in the list
                res
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}