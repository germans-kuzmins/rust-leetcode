use std::collections::HashSet;

struct Rect {
    h: i32,
    x: i32,
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        if filtered.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = filtered.len() - 1;

        while left != right {
            if filtered[left] != filtered[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}
