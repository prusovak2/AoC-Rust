use std::{rc::Rc, cell::{RefCell, Ref, RefMut}, fmt::Debug, io::empty};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>
}

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
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

impl<T : PartialOrd + Debug> List<T>  { 
    pub fn new() -> Self {
        List {head: None}
    }

    pub fn print(&self) {
        let mut cur = self.head.clone();
        while let Some(node) = cur  {
            println!("[{:?}]", node.borrow().elem);
            cur = node.borrow().next.clone();
        }
    }

    pub fn insert(&mut self, elem:T) {
        //let new_node = Node::new(elem);

        let taken_head = self.head.take();
        self.head = self.insert_recursive(taken_head, elem);
    }

    fn insert_recursive(&self, current: Link<T>, elem:T) -> Link<T> {
        match current {
            Some(ref cur_node) => {
                let should_insert = elem <= cur_node.borrow().elem;
                if should_insert {
                    let new_node = Node::new(elem);
                    let prev_node = cur_node.borrow_mut().prev.take();
                    new_node.borrow_mut().prev = prev_node.clone();
                    new_node.borrow_mut().next = Some(cur_node.clone());
                    cur_node.borrow_mut().prev = Some(new_node.clone());
                    if let Some(ref prev_node) = new_node.borrow().prev {
                        prev_node.borrow_mut().next = Some(Rc::clone(&new_node));
                    } 
                    return Some(Rc::clone(&new_node));
                }
                else{
                    let new_next = self.insert_recursive(cur_node.borrow().next.clone(), elem);
                    cur_node.borrow_mut().next = new_next;
                    Some(cur_node.clone())
                }
            }
            None => {
                let new_node = Node::new(elem);
                return  Some(new_node);
            }
        }
    }

    // pub fn insert(&mut self, elem:T) {
    //     match self.head.take() {
    //         Some(mut old_head) => {
    //             // inserting new head to non-empty list
    //             //let mut mut_borrowed_old_head = old_head.borrow_mut();
    //             if  old_head.borrow().elem > elem {
    //                 let new_node = Node::new(elem);
    //                 new_node.borrow_mut().next = Some(old_head.clone());
    //                 old_head.borrow_mut().prev = Some(new_node.clone());
    //                 self.head = Some(new_node);
    //             }
    //             // insert into the middle of the list
    //             else {
    //                 List::insert_impl(&mut old_head, elem);
    //                 self.head = Some(old_head);
    //             }
    //         }
    //         None => {
    //             let new_node = Node::new(elem);
    //             self.head = Some(new_node);
    //         },
    //     }
    // }

    // fn insert_impl(cur: &mut Rc<RefCell<Node<T>>>, elem:T) {
    //     println!("cur: {:?}", cur.borrow().elem);
    //     let mut mut_borrowd_cur = cur.borrow_mut();
    //     match mut_borrowd_cur.next.take() {
    //         Some(next) => {
    //             let mut muttably_borrowed_next: RefMut<'_, Node<T>> = next.borrow_mut();
    //             if mut_borrowd_cur.elem < elem && muttably_borrowed_next.elem >= elem {
    //                 let new_node = Node::new(elem);
    //                 let mut new_mut_borrow = new_node.borrow_mut();
    //                 new_mut_borrow.next = mut_borrowd_cur.next.clone();
    //                 new_mut_borrow.prev = Some(cur.clone());
    //                 mut_borrowd_cur.next = Some(new_node.clone());
    //                 muttably_borrowed_next.prev = Some(new_node.clone());
    //             }
    //             else {
    //                 println!("travering further");
    //                 List::insert_impl(&mut next.clone(), elem);
    //             }
    //         }
    //         None => {
    //             // insert at the end of list
    //             let new_node = Node::new(elem);
    //             new_node.borrow_mut().prev = Some(cur.clone());
    //             mut_borrowd_cur.next = Some(new_node);
    //             return;
    //         },
    //     }
    // }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn insert_at_beggining() {
        let mut list:List<i32> = List::new();
        list.insert(5);
        list.insert(4);
        list.insert(3);
        list.insert(2);
        list.insert(1);

        list.print();
    }

    #[test]
    fn insert_middle() {
        let mut list:List<i32> = List::new();
        list.insert(6);
        list.insert(5);
        list.insert(2);
        list.insert(1);

        println!("List a");
        list.print();

        list.insert(4);

        println!("List b");
        list.print();
    }
}