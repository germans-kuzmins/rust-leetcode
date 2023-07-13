// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut result = i32::MAX;
    while left <= right {
        if nums[left] <= nums[right] {
            result = result.min(nums[left]);
        }
        let middle: usize = left + (right - left) / 2;
        result = result.min(nums[middle]);
        if nums[middle] >= nums[left] {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    
    return result;
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given
        let nums = vec![3, 4, 5, 1, 2];

        // When
        let result = find_min(nums);

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn case2() {
        // Given
        let nums = vec![4, 5, 6, 7, 0, 1, 2];

        // When
        let result = find_min(nums);

        // Then
        assert_eq!(result, 0);
    }

    #[test]
    fn case3() {
        // Given
        let nums = vec![11, 13, 15, 17];

        // When
        let result = find_min(nums);

        // Then
        assert_eq!(result, 11);
    }

    #[test]
    fn case4() {
        // Given
        let nums = vec![1];

        // When
        let result = find_min(nums);

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn case5() {
        // Given
        let nums = vec![3,1,2];

        // When
        let result = find_min(nums);

        // Then
        assert_eq!(result, 1);
    }
}