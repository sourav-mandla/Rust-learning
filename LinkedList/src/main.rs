#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn add_node(&mut self , value: i32) {

        let mut temp = self;

        while let Some(ref mut x) = temp.next {
          temp = x;
        }
        temp.next = Some(Box::new(ListNode::new(value))); 
    }
}
// taken from https://users.rust-lang.org/t/a-problem-about-pointer/32319
pub fn delete_duplicates(head: Option<ListNode>) -> Option<ListNode> {
    match head {
        Some(mut node) => {
            let mut value = node.val;
            let mut ptr =  &mut node;

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
                if ptr.next.as_ref().map_or(false, |n| n.val == value){
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
    let mut head = ListNode::new(5);
    head.add_node(5);
    head.add_node(6);
    head.add_node(6);
    head.add_node(6);
    head.add_node(7);
    head.add_node(7);
    println!("List is {:?}", head);
    //let mut head = Some(Box::new(ListNode::new(4)));
    let mut head = Some(head);
    head = delete_duplicates(head);
    //let mut head = Some(Box::new(head));
    //delete_duplicates(&mut head);
    println!("List is {:#?}", head.unwrap());
}