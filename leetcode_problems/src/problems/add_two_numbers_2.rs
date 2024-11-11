// Definition for singly-linked list.
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
}

fn create_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    for x in list {
        *current = Some(Box::new(ListNode::new(x)));
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let ln1 = ListNode::new(0);
    println!("{ln1:?}");

    return create_list(vec![8, 9, 9, 9, 0, 0, 0, 1]);
}

pub fn test_add_two_numbers() {
    let result = add_two_numbers(
        create_list(vec![9, 9, 9, 9, 9, 9, 9]),
        create_list(vec![9, 9, 9, 9]),
    );
    println!("{result:?}");
}
