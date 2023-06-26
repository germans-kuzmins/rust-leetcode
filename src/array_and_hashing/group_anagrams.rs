use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hash_map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
    for s in strs {
        let mut key = [0 as u8; 26];
        for c in s.chars() {
            key[c as usize - 'a' as usize] +=1;
        }
        if let Some(existing_key) = hash_map.get_mut(&key) {
            existing_key.push(s);
        } else {
            hash_map.insert(key, vec![s]);
        }
    }
    return hash_map.into_values().collect();
}

mod tests {
    use super::group_anagrams;

    #[test]
    fn case1() {
        // Given:
        let anagrams: Vec<String> = vec![
            "eat".to_owned(),
            "tea".to_owned(),
            "tan".to_owned(),
            "ate".to_owned(),
            "nat".to_owned(),
            "bat".to_owned(),
        ];

        // When:
        let mut grouped_anagrams = group_anagrams(anagrams);

        // Then:
        let mut expected = vec![
            vec!["bat".to_owned()],
            vec!["nat".to_owned(), "tan".to_owned()],
            vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()],
        ];
        assert_eq!(grouped_anagrams.sort().clone(), expected.sort().clone());
    }

    #[test]
    fn case2() {
        // Given:
        let anagrams: Vec<String> = vec!["".to_owned()];

        // When:
        let grouped_anagrams = group_anagrams(anagrams);

        // Then:
        let expected = vec![vec!["".to_owned()]];
        assert_eq!(grouped_anagrams, expected);
    }

    #[test]
    fn case3() {
        // Given:
        let anagrams: Vec<String> = vec!["a".to_owned()];

        // When:
        let grouped_anagrams = group_anagrams(anagrams);

        // Then:
        let expected = vec![vec!["a".to_owned()]];
        assert_eq!(grouped_anagrams, expected);
    }
}
