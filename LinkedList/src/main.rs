#[derive(Debug)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    #[inline]
    fn new(val: i32) -> Self {
        Node { next: None, val }
    }
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }
    fn add_node(&mut self, value: i32) {
        match self.head.as_mut() {
            None => {
                self.head = Some(Box::new(Node::new(value)));
            }
            Some(first_node) => {
                let mut temp = first_node;
                while let Some(ref mut x) = temp.next {
                    temp = x;
                }
                temp.next = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.head.as_ref().is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_some() {
            let mut temp = self.head.as_mut();
            while temp
                .as_mut()
                .unwrap()
                .next
                .as_ref()
                .map_or(false, |node| node.next.is_some())
            {
                temp = temp.unwrap().next.as_mut();
            }

            let ret_val = Some(temp.as_ref().unwrap().next.as_ref().unwrap().val);
            temp.as_mut().unwrap().next = None;
            ret_val
        } else {
            let ret_val = Some(self.head.as_ref().unwrap().val);
            self.head = None;
            ret_val
        }
    }
}
// taken from https://users.rust-lang.org/t/a-problem-about-pointer/32319
pub fn delete_duplicates(head: Option<Node>) -> Option<Node> {
    match head {
        Some(mut node) => {
            let mut value = node.val;
            let mut ptr = &mut node;

            /* while let Some(ref mut n) = ptr.next {
                if value == n.val {
                    ptr.next = n.next.clone();
                } else {
                    value = n.val;
                   // problem in right here.
                   // the error of complier : E0308: mismatched types  expected mutable reference, found struct `std::boxed::Box`  note: expected type `&mut std::boxed::Box<ListNode>`found type `std::boxed::Box<ListNode>`
                   // i know it what is meaning, but i can't solve it.
                    ptr = n;
                }
            } */
            while ptr.next.is_some() {
                if ptr.next.as_ref().map_or(false, |n| n.val == value) {
                    ptr.next = ptr.next.as_mut().and_then(|n| n.next.take());
                } else {
                    value = ptr.next.as_ref().unwrap().val;
                    ptr = ptr.next.as_mut().unwrap();
                }
            }
            Some(node)
        }
        None => {
            println!("None reached");
            None
        }
    }
}
fn main() {
    let mut list = LinkedList::new();
    list.add_node(5);
    list.add_node(6);
    list.add_node(6);
    list.add_node(6);
    list.add_node(7);
    list.add_node(7);
    println!("List is {:#?}", list);
    //let mut head = Some(Box::new(ListNode::new(4)));
    //let mut head = Some(head);
    //head = delete_duplicates(head);
    //let mut head = Some(Box::new(head));
    //delete_duplicates(&mut head);
    println!("======================================");
    println!("deleted node:{}", list.pop().unwrap());
    println!("deleted node:{}", list.pop().unwrap());
    println!("deleted node:{}", list.pop().unwrap());
    println!("deleted node:{}", list.pop().unwrap());
    println!("deleted node:{}", list.pop().unwrap());
    println!("deleted node:{}", list.pop().unwrap());
    //println!("deleted node:{}", list.pop().unwrap());

    println!("Remaining List is:");
    println!("{:#?}", list);
    //println!("deleted node:{}", list.pop().unwrap());
    println!("======================================");
}
