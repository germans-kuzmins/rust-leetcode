use std::collections::VecDeque;

//https://leetcode.com/problems/longest-substring-without-repeating-characters
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut result = 0;
    let mut set: VecDeque<char> = VecDeque::new();


    for c in s.chars() {
        while set.contains(&c) {
            set.pop_front();
        }
        set.push_back(c);
        result = result.max(set.len())
    }
    
    return result as i32;
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given
        let s = String::from("abcacbb");

        // When
        let result = length_of_longest_substring(s);

        // Then
        assert_eq!(result, 3);
    }

    #[test]
    fn case2() {
        // Given
        let s = String::from("bbbbb");

        // When
        let result = length_of_longest_substring(s);

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn case3() {
        // Given
        let s = String::from("pwwkew");

        // When
        let result = length_of_longest_substring(s);

        // Then
        assert_eq!(result, 3);
    }

    #[test]
    fn case4() {
        // Given
        let s = String::from("aab");

        // When
        let result = length_of_longest_substring(s);

        // Then
        assert_eq!(result, 2);
    }
}
