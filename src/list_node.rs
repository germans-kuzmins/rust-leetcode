#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn vec_to_list_node(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current: Option<Box<ListNode>> = None;

    for &val in vec.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = current;
        current = Some(Box::new(node));
    }

    current
}

pub fn from_node(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    if let Some(node) = head {
        let mut current = Some(node);
        while let Some(mut boxed_node) = current {
            vec.push(boxed_node.val);
            current = boxed_node.next.take();
        }
    } else {
        return vec;
    }
    vec
}