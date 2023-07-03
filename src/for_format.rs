use std::collections::HashSet;

impl Solution {
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
}
