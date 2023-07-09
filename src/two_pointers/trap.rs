// https://leetcode.com/problems/trapping-rain-water/
pub fn trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }
    let (mut left, mut right) = (0, height.len() - 1);
    let (mut left_max, mut right_max) = (height[left], height[right]);
    let mut result = 0;
    while left < right {
        if left_max < right_max  {
            left +=1;
            left_max = left_max.max(height[left]);
            result += left_max - height[left];
        } else {
            right -=1;
            right_max = right_max.max(height[right]);
            result += right_max - height[right];
        }
        
    }
    return result;
}

mod tests {
    use super::*;
    #[test]
    fn case1() {
        // Given:
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

        // When:
        let result = trap(height);

        // Then
        assert_eq!(result, 6);
    }

    #[test]
    fn case2() {
        // Given:
        let height = vec![4, 2, 0, 3, 2, 5];

        // When:
        let result = trap(height);

        // Then
        assert_eq!(result, 9);
    }
}
