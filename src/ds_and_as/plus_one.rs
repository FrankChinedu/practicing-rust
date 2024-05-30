pub struct Solution;

fn is_two_or_more_digits(n: i32) -> bool {
    n.abs() >= 10
}

fn split_two_digit_number(n: i32) -> (i32, i32) {
    let tens = n / 10;
    let units = n % 10;
    (tens, units)
}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        digits.reverse();
        let mut num_to_add = 1;

        for i in 0..digits.len() {
            if num_to_add == 0 {
                break;
            }

            let num = digits[i] + num_to_add;
            if !is_two_or_more_digits(num) {
                digits[i] = num;
                num_to_add = 0;
                break;
            }
            let data = split_two_digit_number(num);
            println!("data {:?}", data);
            digits[i] = data.1;
            num_to_add = data.0
        }

        digits.reverse();
        if num_to_add != 0 {
            digits.insert(0, num_to_add);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let digits = vec![1, 2, 3];
        let result = Solution::plus_one(digits);
        assert_eq!(result, [1, 2, 4].to_vec());
    }

    #[test]
    fn simple_column_number_2() {
        let digits = vec![4, 3, 2, 1];
        let result = Solution::plus_one(digits);
        assert_eq!(result, [4, 3, 2, 2].to_vec());
    }

    #[test]
    fn simple_column_number_3() {
        let digits = vec![9];
        let result = Solution::plus_one(digits);
        assert_eq!(result, [1, 0].to_vec());
    }

    #[test]
    fn simple_column_number_4() {
        let digits = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let result = Solution::plus_one(digits);
        assert_eq!(result, [9, 8, 7, 6, 5, 4, 3, 2, 1, 1].to_vec());
    }
}
