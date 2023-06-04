pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map = std::collections::HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().all(|v: i32| v == 0)
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(is_anagram(String::from("anagram"),String::from("nagaram")), true);
    }

    #[test]
    fn case2() {
        assert_eq!(is_anagram(String::from("rat"),String::from("car")), false);
    }
}