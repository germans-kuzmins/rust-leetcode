/* 
f(0): ""

f(1): "("f(0)")"

f(2): "("f(0)")"f(1), "("f(1)")"

f(3): "("f(0)")"f(2), "("f(1)")"f(1), "("f(2)")"

So f(n) = "("f(0)")"f(n-1) , "("f(1)")"f(n-2) "("f(2)")"f(n-3) ... "("f(i)")"f(n-1-i) ... "(f(n-1)")"
 */
pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<Vec<String>> = vec![];
        result.push(vec![String::from("")]);
        result.push(vec![String::from("()")]);
        if n <= 1 {
            return result[n as usize].clone();
        }

        for i in 2..n + 1 {
            let mut list = vec![];
            for j in 0..i  {
                for first in result[j as usize].clone() {
                    for second in result[(i - 1 - j) as usize].clone() {
                        list.push(format!("({}){}", first, second));
                    }
                }
            }
            result.push(list)
        }
        return result[n as usize].clone(); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let n = 1;

        // When:
        let result = generate_parenthesis(n);

        // Then:
        assert_eq!(result, vec!["()"])
    }

    #[test]
    fn case2() {
        // Given:
        let n = 2;

        // When:
        let result = generate_parenthesis(n);

        // Then:
        assert_eq!(result, vec!["()()", "(())"])
    }
    
    #[test]
    fn case3() {
        // Given:
        let n = 3;

        // When:
        let result = generate_parenthesis(n);

        // Then:
        assert_eq!(result, vec!["()()()", "()(())", "(())()", "(()())", "((()))"])
    }
}