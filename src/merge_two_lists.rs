use super::list_node::*;
// recursive approach
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {

    match (list1, list2) {
        (Some(mut a), Some(mut b)) => {
            if a.val <= b.val {
                a.next = merge_two_lists(a.next, Some(b));
                return Some(a);
            } else {
                b.next = merge_two_lists(Some(a), b.next);
                return Some(b);
            }
        }
        (Some(a_or_b), None) | (None, Some(a_or_b)) => return Some(a_or_b),
        (None, None) => return None,
    }
}

// iterative approcach
pub fn merge_two_lists_itterative(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {

    match (list1.is_some(), list2.is_some()) {
        (true, true) => {
            let mut head = Box::new(ListNode::new(-1));
            let mut tail = head.as_mut();
            let mut left = list1;
            let mut right = list2;
            while let (Some(a), Some(b)) = (left.as_ref(), right.as_ref())  {
                if a.val <= b.val {
                    tail.next = Some(Box::new(ListNode::new(a.val)));
                    left = left.unwrap().next;
                } else {
                    tail.next = Some(Box::new(ListNode::new(b.val)));
                    right = right.unwrap().next;
                }
                tail = tail.next.as_mut().unwrap();
            }
            tail.next = if left.is_some() {left} else {right};
            return head.next;
        }
        (true, false) => return list1,
        (false, true) => return list2,
        (false, false) => return None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let list1 = vec_to_list_node(vec![1, 2, 4]);
        let list2 = vec_to_list_node(vec![1, 3, 4]);

        // When:
        let result = merge_two_lists(list1, list2);

        // Then:
        assert_eq!(result.is_some(), true);
        assert_eq!(from_node(result), vec![1, 1, 2, 3, 4, 4])
    }

    #[test]
    fn case2() {
        // Given:
        let list1 = vec_to_list_node(vec![]);
        let list2 = vec_to_list_node(vec![]);

        // When:
        let result = merge_two_lists(list1, list2);

        // Then:
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn case3() {
        // Given:
        let list1 = vec_to_list_node(vec![]);
        let list2 = vec_to_list_node(vec![0]);

        // When:
        let result = merge_two_lists(list1, list2);

        // Then:
        assert_eq!(result.is_some(), true);
        assert_eq!(from_node(result), vec![0])
    }

    #[test]
    fn case4() {
        // Given:
        let list1 = vec_to_list_node(vec![1, 2, 4]);
        let list2 = vec_to_list_node(vec![1, 3, 4]);

        // When:
        let result = merge_two_lists_itterative(list1, list2);

        // Then:
        assert_eq!(result.is_some(), true);
        assert_eq!(from_node(result), vec![1, 1, 2, 3, 4, 4])
    }
}
