// bad stack was reinventing the Option type: 
// enum Link {
//     Empty,
//     More(Box<Node>),
// } 
// in nothing other than just Option<Box<Node>>
// lets try to rewrite the list using Option

use std::mem;

pub struct List<T> {
    head: Link<T>
} 

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            next: self.head.take() // takes the value of Option, leaving none in the place, effectively doing std::mem::replace(&mut self.head, None) 
        };
        self.head = Some(Box::new(new_node));
    }

    // pub fn pop(&mut self) -> Option<T> {
    //     match self.head.take() { // take
    //         Some(boxed_node) => {
    //             let res = Some(boxed_node.elem);
    //             self.head = boxed_node.next;
    //             res
    //         }
    //         None => None,
    //     }
    // }

    // !!!
    // replace match option { None => None, Some(x) => Some(y) } pattern with map function
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node|{
            self.head = node.next;
            node.elem
        })
    }

    // pub fn peek(&self) -> Option<&T> {
    //     self.head.map(|node| {
    //         &node.elem
    //     })
    // }
    
    // as_ref demotes the Option<T> to an Option<&T>
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    // as_mut demotes the Option<T> to an Option<&mut T>
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}


// IntoIter - T
// IterMut - &mut T
// Iter - &T

// TO IMPLEMENT ITERATOR PATTERN I NEED TO IMPLEMENT:
// * iterator stuct
// * iter (into_iter, iter_mut) method on a data structure I want to iterate, returning an instance of the iterator struct
// * Iterator trait for iterator struct


// IntoIter is jusy like calling pop over and over
pub struct IntoIter<T>(List<T>); // tuple struct

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter
// Lifetimes
// // Only one reference in input, so the output must be derived from that input
// fn foo(&A) -> &B; // sugar for:
// fn foo<'a>(&'a A) -> &'a B;

// // Many inputs, assume they're all independent
// fn foo(&A, &B, &C); // sugar for:
// fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

// // Methods, assume all output lifetimes are derived from `self`
// fn foo(&self, &B, &C) -> &D; // sugar for:
// fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;

// Iter is generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// No lifetime here, List doesn't have any associated lifetimes
impl<T> List<T> {
    // We declare a fresh lifetime here for the *exact* borrow that
    // creates the iter. Now &self needs to be valid as long as the
    // Iter is around.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            // following two lines are equivalent
            //next: self.head.as_ref().map(|node| &**node)
            next: self.head.as_deref() //Leaves the original Option in-place, 
            //creating a new one with a reference to the original one, additionally coercing the contents via Deref
        }
    }
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            // following three lines are equivalent
            //self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            //self.next = node.next.as_ref().map(|node| &**node);
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// ITER MUT 
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}