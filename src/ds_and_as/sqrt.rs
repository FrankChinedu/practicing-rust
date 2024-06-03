pub struct Solution;

const MAX_X: u32 = 2147483647;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low = 0u32;
        let mut high = MAX_X;
        loop {
            let i: u32 = (high + low) / 2;
            let a = i * i <= x as u32;
            let b = (i + 1) * (i + 1) > x as u32;

            if a && b {
                return i as i32;
            }
            if a {
                low = i;
            } else {
                high = i;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let x = 8;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn simple_column_number_2() {
        let x = 4;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 2);
    }
}
