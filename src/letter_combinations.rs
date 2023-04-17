use std::{collections::VecDeque};

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }
    
    let mut dequeue: VecDeque<String> = VecDeque::new();
    let pad: [&str; 10] = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    dequeue.push_back(String::from(""));

    for (index, c) in digits.char_indices() {
        let digit = c.to_digit(10).unwrap() as usize;
        
        let digit_for_pad = pad[digit];
        
        while dequeue.front().unwrap().len() == index {
            let element = dequeue.pop_front().unwrap();
            for x in digit_for_pad.chars() {
                dequeue.push_back( format!("{}{}", element, x));
            }
        }
    }

    return dequeue.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let digits = String::from("23");

        // When:
        let result = letter_combinations(digits);

        // Then:
        let expected_result: Vec<String> = vec![
            String::from("ad"),
            String::from("ae"),
            String::from("af"),
            String::from("bd"),
            String::from("be"),
            String::from("bf"),
            String::from("cd"),
            String::from("ce"),
            String::from("cf"),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn case2() {
        // Given:
        let digits = String::from("");

        // When:
        let result = letter_combinations(digits);

        // Then:
        let expected_result: Vec<String> = vec![];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn case3() {
        // Given:
        let digits = String::from("2");

        // When:
        let result = letter_combinations(digits);

        // Then:
        let expected_result: Vec<String> =
            vec![String::from("a"), String::from("b"), String::from("c")];
        assert_eq!(result, expected_result);
    }
}
