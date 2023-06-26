pub fn longest_palindrome(s: String) -> String {

    let mut dp = vec![vec![false; s.len()]; s.len()];
    for i in 0..s.len() {
        dp[i][i] = true;
    }
    let mut max_length = 1;
    let mut start = 0;
    let bytes = s.as_bytes();

    for i in 0..s.len() - 1 {
        if bytes[i] == bytes[i+1] {
            dp[i][i + 1] = true;
            start = i;
            max_length = 2;
        }
    }

    let mut k = 3;
    while k <= s.len() {
        let mut i = 0;
        while i < (s.len() - k + 1) {
            let j = i + k - 1;
            if dp[i + 1][j - 1] &&  bytes[i] == bytes[j] {
                dp[i][j] = true;
                if k > max_length {
                    start = i;
                    max_length = k;
                }
            }
            i = i + 1;
        }

        k = k + 1;
    }
    let end = start + max_length;
    return s[start..end].to_string();
}

#[cfg(test)]
mod tests {
    use super::longest_palindrome;

    #[test]
    fn example1() {
        assert_eq!(longest_palindrome(String::from("babad")), "bab");
    }

    #[test]
    fn example2() {
        assert_eq!(longest_palindrome(String::from("cbbd")), "bb");
    }

    #[test]
    fn example3() {
        assert_eq!(longest_palindrome(String::from("forgeeksskeegfor")), "geeksskeeg");
    }

    #[test]
    fn example4() {
        assert_eq!(longest_palindrome(String::from("Geeks")), "ee");
    }
}

