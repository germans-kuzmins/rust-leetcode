pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut sorted = strs.to_vec();
    sorted.sort();

    let first_string = sorted.first().unwrap();
    let last_string = sorted.last().unwrap();

    let min_range = std::cmp::min(first_string.len(), last_string.len());
    let first_bytes = first_string.as_bytes();
    let second_bytes = last_string.as_bytes();
    let mut result:Vec<u8> = vec![];

    for i in 0..min_range {
        if first_bytes[i] != second_bytes[i] {
            break;
        }
        result.push(first_bytes[i])
    }    
    return String::from_utf8(result).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs: Vec<String> = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(strs), "fl");
    }

    #[test]
    fn case2() {
        let strs: Vec<String> = vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string(),
        ];
        assert_eq!(longest_common_prefix(strs), "");
    }
}
