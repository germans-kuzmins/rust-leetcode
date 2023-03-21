use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::new();
    let bytes = s.as_bytes();
    map.insert('I' as u8, 1);
    map.insert('V' as u8, 5);
    map.insert('X' as u8, 10);
    map.insert('L' as u8, 50);
    map.insert('C' as u8, 100);
    map.insert('D' as u8, 500);
    map.insert('M' as u8, 1000);
    let mut result = 0;
    for i in 0..s.len() {
        let current =map.get(&bytes[i]).unwrap();
        if i < s.len() - 1 {
            let next = map.get(&bytes[i + 1]).unwrap();
            if current < next {
               result -= current;
            } else {
                result += current;
            }
        } else {
            result += current;
        }
        
    }
    return result;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn example3() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
