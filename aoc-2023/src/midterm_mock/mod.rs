use std::{ops::Add, fmt::{Debug, Display}, cell::RefCell, rc::Rc, collections::btree_map::Values};

// ex 1
mod odd_module {
    pub const CONSTANT: i32 = 123;
}

mod even_module {
    pub const CONSTANT: i32 = 246;
}

mod getter_function {
    pub fn get_constant(value: u32) -> i32 {
        if value % 2 == 0 {
            super::odd_module::CONSTANT
        }
        else {
            super::even_module::CONSTANT
        }
    }
}

// ex 2
trait CloneAndDouble {
    fn clone_and_double(&self) -> Self;
}

impl<T: Clone + Add<Output = T>> CloneAndDouble for T  {
    fn clone_and_double(&self) -> Self {
        let clone = self.clone();
        clone + self.clone()
    }
}

// ex 3
trait Unknown {
    fn serialize(&self) -> String;
}

impl Unknown for i32 {
    fn serialize(&self) -> String {
        format!("{self}")
    }
}

impl Unknown for String {
    fn serialize(&self) -> String {
        self.clone()
    }
}

impl<T: Debug> Unknown for Vec<T> {
    fn serialize(&self) -> String {
        let mut res = "[".to_owned();
        for item in self {
            res.push_str(&format!("{:?}, ", item))
        }
        res.push_str("]");
        res
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
    let v: Vec<Box<dyn Unknown>> = Vec::new();
    v
}

fn print_vec(vec: &Vec<Box<dyn Unknown>>) {
    for item in vec {
        let str = item.serialize();
        println!("{}", str)
    }
}

// ex 4
struct BinIter {
    digits: Vec<bool>,
    cur_idx: usize,
    len: usize
}

impl BinIter {
    fn new(number: u32, len: usize) -> BinIter {
        let bin_string = format!("{:b}", number);
        println!("bin string {:?}", bin_string);
        let mut digits: Vec<bool> = Vec::new(); 
        for c in bin_string.chars() {
            if c == '1' {
                digits.push(true)
            }
            else if c == '0' {
                digits.push(false)
            }
            else {
                panic!("unexpected digit {}", c);
            }
        }
        digits.reverse();
        println!("digits: {:?}", digits);
        BinIter {digits, cur_idx:0, len:len}
    }
}

impl  Iterator for BinIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx >= self.len {
            return None;
        }
        else{
            let item = self.digits[self.cur_idx];
            self.cur_idx +=1;
            return Some(item);
        }
    }
}

// ex 5
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem:T,
    prev: Link<T>,
    next: Link<T>
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elem)
    }
}

impl<T:Display>  Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut cur_head = self.head.clone();
        while let Some(cur_node) = cur_head {
            write!(f, "{}, ", cur_node.borrow().elem)?;
            cur_head = cur_node.borrow().next.clone();
        }
        write!(f, "]")
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "[")?;
       let mut cur_head = self.head.clone();
       while let Some(cur_node) = cur_head {
            write!(f, "{:?}, ", cur_node.borrow().elem)?;
            cur_head = cur_node.borrow().next.clone()
       }
       write!(f, "]")
    }
}

impl<T: PartialEq> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut first_head = self.head.clone();
        let mut second_head = other.head.clone();
        loop {
            match (first_head, second_head) {
                (None, None) =>  {return true},
                (None, Some(_)) => return false,
                (Some(_), None) => return false,
                (Some(first_node), Some(second_node)) => {
                    if first_node.borrow().elem != second_node.borrow().elem {
                        return  false;
                    }
                    first_head = first_node.borrow().next.clone();
                    second_head = second_node.borrow().next.clone();
                },
            }
        }
    }
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

impl<T: Display + Clone> List<T> {
    fn new() -> Self {
        List { head: None, tail: None, size: 0 }
    }

    fn print_list(&self) {
        println!("LIST: {}", self)
    }

    fn push(&mut self, elem: T) {
        match self.head.take() {
            Some(old_head) => {
                let new_node = Node::new(elem);
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head); // clone?
                self.head = Some(new_node);
                self.size +=1;
            },
            None => {
                // empty list
                let new_node = Node::new(elem);
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
                self.size +=1 ;
            },
        }
    }

    fn push_back(&mut self, elem: T) {
        match self.tail.take() {
            Some(old_tail) => {
                let new_node = Node::new(elem);
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail =Some(new_node);
                self.size +=1;
            },
            None => {
                // empty list
                let new_node = Node::new(elem);
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
                self.size +=1;
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.borrow().next.clone();
            let value = old_head.borrow().elem.clone();
            match old_head.clone().borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                },
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            self.size -= 1;
            value
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            self.tail = old_tail.borrow().prev.clone();
            let val = old_tail.borrow().elem.clone();
            match old_tail.clone().borrow_mut().prev.take(){
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail)
                },
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }

            self.size -=1;
            val
        })
    }
}

// ex 6
type GraphLink<T> = Rc<RefCell<GraphNode<T>>>;

struct GraphNode<T> {
    elem: T,
    neighbors: Vec<GraphLink<T>>
}

struct Graph<T> {
    nodes: Vec<GraphLink<T>>
}

impl<T> GraphNode<T> {
    fn new(elem: T, neighbors: Vec<GraphLink<T>>) -> GraphNode<T> {
        GraphNode {elem: elem, neighbors: neighbors}
    }

    fn get_value(&self) -> &T {
        &self.elem
    }
}

impl<T: Debug> Debug for GraphNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[value:{:?}, adjacents: [", self.elem)?;
        for node in &self.neighbors {
            write!(f, "{:?}, ", node.borrow().elem)?;
        }
        write!(f, "]]")
    }
}

impl<T: Debug> Graph<T> {
    fn new(nodes: Vec<GraphLink<T>>) -> Self {
        Graph { nodes: nodes }
    }

    fn dfs(&self, start_node: GraphLink<T>) -> Vec<GraphLink<T>> {
        let mut visited: Vec<GraphLink<T>> = Vec::new();
        Graph::visit_node(start_node, &mut visited);
        visited
    }

    fn visit_node(node: GraphLink<T>, visited: &mut Vec<GraphLink<T>>){
        println!("visiting {:?}", node.borrow().elem);
        visited.push(node.clone());
        for adjacent in node.borrow().neighbors.iter() {
            Graph::visit_node(adjacent.clone(), visited);
        }
    }
}

// ex 7
// trait Task {
//     fn execute(&self) -> usize;
// }

// struct SumTask{
//     n1: usize,
//     n2: usize
// }

// impl SumTask {
//     fn new(n1: usize, n2: usize) -> Self {
//         SumTask {n1: n1, n2: n2}
//     }
// }

// impl  Task for SumTask {
//     fn execute(&self) -> usize {
//         self.n1 + self.n2
//     }
// }

// struct LenTask {
//     string: String
// }

// impl LenTask {
//     fn new(s: String) ->Self {
//         LenTask {string:s}
//     }
// }

// impl Task for LenTask {
//     fn execute(&self) -> usize {
//         self.string.len()
//     }
// }

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq)]
    struct TestData {
        value: i32,
    }

    use std::io::Write;
    use super::*;
    
    #[test]
    fn test_graph_dfs() {
        // Create nodes
        let node1 = Rc::new(RefCell::new(GraphNode::new(TestData { value: 1 }, Vec::new())));
        let node2 = Rc::new(RefCell::new(GraphNode::new(TestData { value: 2 }, Vec::new())));
        let node3 = Rc::new(RefCell::new(GraphNode::new(TestData { value: 3 }, Vec::new())));
        let node4 = Rc::new(RefCell::new(GraphNode::new(TestData { value: 4 }, Vec::new())));
        let node5 = Rc::new(RefCell::new(GraphNode::new(TestData { value: 5 }, Vec::new())));
    
        // Connect nodes to form a graph
        node1.borrow_mut().neighbors.push(Rc::clone(&node2));
        node1.borrow_mut().neighbors.push(Rc::clone(&node3));
        node2.borrow_mut().neighbors.push(Rc::clone(&node4));
        node3.borrow_mut().neighbors.push(Rc::clone(&node5));
    
        let graph = Graph::new(vec![Rc::clone(&node1), Rc::clone(&node2), Rc::clone(&node3), Rc::clone(&node4), Rc::clone(&node5)]);
    
        // Perform DFS starting from node1
        let result = graph.dfs(Rc::clone(&node1));
        println!("{:?}", result);
    
        // Verify the order of visited nodes
        //assert_eq!(result, vec![Rc::clone(&node1), Rc::clone(&node2), Rc::clone(&node4), Rc::clone(&node3), Rc::clone(&node5)]);
    }

    #[test]
    fn test_list_push_back_pop_back() {
        let mut list = List::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_list_push_pop() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_print() {
        let mut list = List::new();

        list.push("Hello");
        list.push("World");
        list.push("Rust");

        list.print_list();
    }

    #[test]
    fn test_list_eq() {
        let mut list1 = List::new();
        list1.push(1);
        list1.push(2);
        list1.push(3);

        let mut list2 = List::new();
        list2.push(1);
        list2.push(2);
        list2.push(3);

        assert_eq!(list1, list2);

        list2.push(4);

        assert_ne!(list1, list2);
    }

    #[test]
    fn test_bin_iter() {
        let bin_iter = BinIter::new(10, 4);
        let result: Vec<bool> = bin_iter.collect();
        assert_eq!(result, vec![false, true, false, true]);

        let bin_iter = BinIter::new(42, 6);
        let result: Vec<bool> = bin_iter.collect();
        assert_eq!(result, vec![false, true, false, true, false, true]);

        let bin_iter = BinIter::new(7, 3);
        let result: Vec<bool> = bin_iter.collect();
        assert_eq!(result, vec![true, true, true]);
    }

    #[test]
    fn test_i32_serialize() {
        let value: i32 = 42;
        assert_eq!(value.serialize(), "42");
    }

    #[test]
    fn test_string_serialize() {
        let value: String = "hello".to_owned();
        assert_eq!(value.serialize(), "hello");
    }

    #[test]
    fn test_vec_serialize() {
        let vec: Vec<i32> = vec![1, 2, 3];
        assert_eq!(vec.serialize(), "[1, 2, 3, ]");

        let vec: Vec<String> = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!(vec.serialize(), "[\"a\", \"b\", ]");
    }

    #[test]
    fn test_get_vec() {
        let vec = get_vec();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_print_vec() {
        let vec: Vec<Box<dyn Unknown>> = vec![
            Box::new(42),
            Box::new("hello".to_owned()),
            Box::new(vec![1, 2, 3]),
        ];

        // Since we are printing, we'll just visually verify the output
        print_vec(&vec);
    }
}
