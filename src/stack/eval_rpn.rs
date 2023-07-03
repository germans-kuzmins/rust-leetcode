pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for token in tokens {
        if token == "+" || token == "-" || token == "*" || token == "/" {
            let operand2 = stack.pop().unwrap();
            let operand1 = stack.pop().unwrap();

            let result = match token.as_ref() {
                "+" => operand1 + operand2,
                "-" => operand1 - operand2,
                "*" => operand1 * operand2,
                "/" => operand1 / operand2,
                _ => panic!("Invalid token: {}", token),
            };

            stack.push(result);
        } else {
            stack.push(token.parse().unwrap());
        }
    }

    stack.pop().unwrap()
}

mod tests {
    use super::*;
    use crate::vec_of_strings; 

    #[test]
    fn case1() {
        // Given:
        let mut tokens = vec_of_strings!["2","1","+","3","*"];

        // When:
        let result = eval_rpn(tokens);

        // Then:
        assert_eq!(result, 9);
    }

    #[test]
    fn case2() {
        // Given:
        let mut tokens = vec_of_strings!["4","13","5","/","+"];

        // When:
        let result = eval_rpn(tokens);

        // Then:
        assert_eq!(result, 6);
    }

    #[test]
    fn case3() {
        // Given:
        let mut tokens = vec_of_strings!["10","6","9","3","+","-11","*","/","*","17","+","5","+"];

        // When:
        let result = eval_rpn(tokens);

        // Then:
        assert_eq!(result, 22);
    }
}
