use super::list_node::*;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });

    let mut fast = dummy.clone();
    let mut slow = dummy.as_mut();

    for _ in 0..n {
        fast = fast.next.unwrap();
    }

    while let Some(node) = fast.next {
        fast = node;
        slow = slow.next.as_mut().unwrap();
    }

    slow.next = slow.next.as_mut().unwrap().next.clone();

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let nums = vec![1, 2, 3, 4, 5];
        let node = vec_to_list_node(nums);

        // When:
        let result = remove_nth_from_end(node, 2);

        // Then:
        assert_eq!(result.is_some(), true);
        assert_eq!( from_node(result), vec![1, 2, 3, 5])
    }

    #[test]
    fn case2() {
        // Given:
        let nums = vec![1];
        let node = vec_to_list_node(nums);

        // When:
        let result = remove_nth_from_end(node, 1);

        // Then:
        assert_eq!(result.is_some(), false);
    }
    #[test]
    fn case3() {
        // Given:
        let nums = vec![1, 2];
        let node = vec_to_list_node(nums);

        // When:
        let result = remove_nth_from_end(node, 1);

        // Then:
        assert_eq!(result.is_some(), true);
        assert_eq!( from_node(result), vec![1])
    }
}
