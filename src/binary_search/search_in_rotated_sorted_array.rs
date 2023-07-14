//https://leetcode.com/problems/search-in-rotated-sorted-array/

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right {
        let middle: usize = left + (right - left) / 2;
        let value: i32 = nums[middle];
        if value == target {
            return middle as i32;
        }

        // [5, 6, 7, 0, 1, 2, 4]
        if nums[left] <= value {
            if target < nums[left] || target > value {
                left = middle + 1;
            } else {
                right = middle - 1
            }
        } else {
            if target < value || target > nums[right] {
                right = middle - 1;
            } else {
                left = middle + 1
            }
        }
    }

    return -1;       
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;

        // When
        let result = search(nums, target);

        // Then
        assert_eq!(result, 4);
    }

    #[test]
    fn case2() {
        // Given
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;

        // When
        let result = search(nums, target);

        // Then
        assert_eq!(result, -1);
    }

    #[test]
    fn case3() {
        // Given
        let nums = vec![1];
        let target = 0;

        // When
        let result = search(nums, target);

        // Then
        assert_eq!(result, -1);
    }

    #[test]
    fn case4() {
        // Given
        let nums = vec![5,1,3];
        let target = 3;

        // When
        let result = search(nums, target);

        // Then
        assert_eq!(result, 2);
    }

    #[test]
    fn case5() {
        // Given
        let nums = vec![5,1,3];
        let target = 5;

        // When
        let result = search(nums, target);

        // Then
        assert_eq!(result, 0);
    }
}