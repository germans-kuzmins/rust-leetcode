struct MinStack {
    main_stack: Vec<i32>,
    min_stack: Vec<i32>
}

impl MinStack {
    fn new() -> Self {
        return MinStack { main_stack: vec![], min_stack:vec![] };
    }

    fn push(&mut self, val: i32) {
        self.main_stack.push(val);
        if self.min_stack.is_empty() {
            self.min_stack.push(val);
            return;
        }
        if let Some(top) = self.min_stack.last() {
            if top >= &val {
                self.min_stack.push(val);
            }
        }
    }

    fn pop(&mut self) {
        let poped = self.main_stack.pop().unwrap();
        if poped == self.get_min() {
            self.min_stack.pop();
        }
    }

    fn top(&mut self) -> i32 {
        *self.main_stack.last().unwrap()
        
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let mut stack = MinStack::new();

        // When:
        stack.push(-2);
        stack.push(0);
        stack.push(-3);

        // Then:
        assert_eq!(stack.get_min(), -3);

        // When:
        stack.pop();

        // Then:
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn case2() {
        // Given:
        let mut stack = MinStack::new();

        // When:
        stack.push(0);
        stack.push(1);
        stack.push(0);

        // Then:
        assert_eq!(stack.get_min(), 0);

        // When:
        stack.pop();

        // Then:
        assert_eq!(stack.top(), 1);
        assert_eq!(stack.get_min(), 0);
    }
}
