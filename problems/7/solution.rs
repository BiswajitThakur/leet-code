impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result: i32 = 0;
        let mut inp: i32 = x;
        loop {
            if inp == 0 {
                return result;
            };
            if result > std::i32::MAX/10 || result < std::i32::MIN/10  {
                return 0;
            };
            result = (result*10) + (inp%10);
            inp /=10;
        }
    }
}

