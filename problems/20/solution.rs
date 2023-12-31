impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for i in s.chars() {
            if i == '(' || i == '{' || i == '[' {
                stack.push(i);
                continue;
            };
            match i {
                ')' => {
                    let v = stack.get(stack.len()-1);
                    if v.is_none() {
                        return false;
                    } else if &'(' == v.unwrap() {
                        stack.pop();
                        continue;
                    } else {
                        return false;
                    }
                },
                '}' => {
                    let v = stack.get(stack.len()-1);
                    if v.is_none() {
                        return false;
                    } else if &'{' == v.unwrap() {
                        stack.pop();
                    } else {
                        return false;
                    }
                },
                ']' => {
                    let v = stack.get(stack.len()-1);
                    if v.is_none() {
                        return false;
                    } else if &'[' == v.unwrap() {
                        stack.pop();
                    } else {
                        return false;
                    }
                },
                _ => return false,
            }
        }
        if stack.len() == 0 {
            true
        } else {
            false
        }
    }
}

