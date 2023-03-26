pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    return vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let nums = vec![-1, 0, 1, 2, -1, -4];

        // When:
        let result = three_sum(nums);

        // Then:
        let expectedResult: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result, expectedResult);
    }

    #[test]
    fn case2() {
        // Given:
        let nums = vec![-1, 0, 1, 2, -1, -4];

        // When:
        let result = three_sum(nums);

        // Then:
        let expectedResult: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result, expectedResult);
    }
}
