use super::list_node::*;
use std::collections::BinaryHeap;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut priority_queue: BinaryHeap<ListNode> = BinaryHeap::new();

    for node in lists.into_iter() {
        if let Some(mut i) = node {
            let mut current: &mut ListNode = i.as_mut();
            priority_queue.push(ListNode::new(current.val));

            while let Some(ref mut next) = current.next {
                priority_queue.push(*next.clone());
                current = next;
            }
        }
    }

    let mut current: Option<Box<ListNode>> = None;

    while !priority_queue.is_empty() {
        if let Some(pop) = priority_queue.pop() {
            let mut node = ListNode::new(pop.val);
            node.next = current;
            current = Some(Box::new(node));
        }
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let list1 = vec_to_list_node(vec![1, 4, 5]);
        let list2 = vec_to_list_node(vec![1, 3, 4]);
        let list3 = vec_to_list_node(vec![2, 6]);

        // When:
        let result: Option<Box<ListNode>> = merge_k_lists(vec![list1, list2, list3]);

        // Then:
        assert_eq!(result.is_some(), true);
        assert_eq!(from_node(result), vec![1, 1, 2, 3, 4, 4, 5, 6])
    }

    #[test]
    fn case2() {
        // Given:
        let list = vec![];

        // When:
        let result = merge_k_lists(list);

        // Then:
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn case3() {
        // Given:
        let list = vec_to_list_node(vec![]);

        // When:
        let result = merge_k_lists(vec![list]);

        // Then:
        assert_eq!(result.is_none(), true);
    }
}
