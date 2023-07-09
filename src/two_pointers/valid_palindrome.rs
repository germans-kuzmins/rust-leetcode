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

    while left < right {
        if filtered[left] != filtered[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
mod test {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let input = "A man, a plan, a canal: Panama".to_string();

        // When:
        let result = is_palindrome(input);

        // Then:
        assert_eq!(result, true);
    }

    #[test]
    fn case2() {
        // Given:
        let input = "race a car".to_string();

        // When:
        let result = is_palindrome(input);

        // Then:
        assert_eq!(result, false);
    }

    #[test]
    fn case3() {
        // Given:
        let input = " ".to_string();

        // When:
        let result = is_palindrome(input);

        // Then:
        assert_eq!(result, true);
    }

    #[test]
    fn case4() {
        // Given:
        let input = "aa".to_string();

        // When:
        let result = is_palindrome(input);

        // Then:
        assert_eq!(result, true);
    }
}
