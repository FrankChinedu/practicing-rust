pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let mut right = x;
        while left <= right {
            let mid = ((left + right) as f64 / 2_f64).floor() as i32;

            if (mid * mid) < x {
                left = mid + 1;
            } else if (mid * mid) > x {
                right = mid - 1
            } else {
                return mid;
            }
        }
        right
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
