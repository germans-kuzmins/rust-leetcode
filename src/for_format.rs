use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hasMap: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = [0 as u8; 26];
            for c in s.chars() {
                key[c as usize - 'a' as usize] += 1;
            }
            if let Some(existing_key) = hasMap.get_mut(&key) {
                existing_key.push(s);
            } else {
                hasMap.insert(key, vec![s]);
            }
        }
        return hasMap.into_values().collect();
    }
}
