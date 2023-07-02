pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut prefix = 1;
    
    for i in 0..nums.len() {
        result.push(prefix);
        prefix = prefix * nums[i];
    }
    let mut postfix = 1;
    for i in (0..nums.len()).rev() {
        result[i] = postfix * result[i];
        postfix = postfix * nums[i];
    }

    return result
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let nums = vec![1, 2, 3, 4];

        // When:
        let mut result = product_except_self(nums);

        // Then:
        let expected = vec![24, 12, 8, 6];
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        // Given:
        let nums = vec![-1, 1, 0, -3, 3];

        // When:
        let result = product_except_self(nums);

        // Then:
        let expected = vec![0, 0, 9, 0, 0];
        assert_eq!(result, expected);
    }
}
