use std::{rc::Rc, cell::{RefCell, Ref, RefMut}};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
elem: T,
next: Link<T>,
prev: Link<T>
}

pub struct List<T> {
head: Link<T>,
tail: Link<T>
}

impl<T> Node<T> {
fn new(elem: T) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Node {
        elem: elem,
        prev: None,
        next: None
    }))
}
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: None, tail: None}
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head)
            },
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head)
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head ) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head)
                },
                None => {
                    self.tail.take();
                },
            }
            // we wanna drop the old head node
            // proble is that into_inner Consumes the RefCell, returning the wrapped value
            // returns Rc in this case, which is immutable refference and we cannot use it
            // to get ownership of the elem value. 
            // thats why we need try_unwrap, which moves out the contents of an Rc if its refcount is 1
            // in our case it should never fail as no other node should hold the ref to this node anymore
            // by calling ok we convert result to option and thus we dont have to implement debug
            // on the node to unwrap it
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }


    // we need to return Option<Ref<T>> instead of Option<T> because
    // borrow returns Ref<T> and that if we just tried to return &node.borrow().elem
    // we are trying to return &T reference, whose lifetime is tied to a local
    // Ref<T> returned by borrow, not to actuall RefCell (which is necesssary, because Ref counts borroes for RefCell)
    pub fn peek_front(&self) -> Option<Ref<T>> { 
        self.head.as_ref().map(|node|{
            // we need a map here to return Ref<T> instead of Ref<Node<T>>
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node|{
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// INTO ITER
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

// ITER
pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_ref().map(|head| head.borrow()))
    }
}

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = Ref<'a, T>;

//     // fn next(&mut self) -> Option<Self::Item> {
//     //     self.0.take().map(|node_ref| {
//     //         // self.0 = node_ref.next.as_ref().map(|head| head.borrow());
//     //         // Ref::map(node_ref, |node| &node.elem)
//     //         // Splits a Ref into multiple Refs for different components of the borrowed data
//     //         let (next, elem) = Ref::map_split(node_ref, |node| {
//     //             (&node.next, &node.elem)
//     //         });
//     //         self.0 = next.as_ref().map(|head| head.borrow());
//     //         elem
//     //     })
//     // }
// }

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

}


