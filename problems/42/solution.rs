impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        if height.len() < 3 {
            return 0
        };
        let mut beg: usize = 0;
        let mut end: usize = 1;
        while end < height.len() {
            if height[beg] < height[end] {
                beg += 1;
                end = beg + 1;
                continue;
            };
            while (end < height.len()) && (height[beg] > height[end]) {
                end += 1;
            }
            if end >= height.len() {
                let mut b: usize = height.len()-1;
                let mut e: usize = b-1;
                while beg < b {
                    if height[b] < height[e] {
                        b -= 1;
                        e = b-1;
                        continue;
                    };
                    while (beg<b) && (height[b] > height[e]) {
                        e -= 1;
                    };
                    let m: i32 = std::cmp::min(height[b], height[e]);
                    for j in e+1..b {
                        result += m - height[j]
                    };
                    b = e;
                    e = b-1;
                };
                return result;
            };
            let min_val: i32 = std::cmp::min(height[beg], height[end]);
            for i in beg+1..end {
                result += min_val - height[i];
            };
            beg = end;
            end = beg + 1;
        };
        result
    }
}

