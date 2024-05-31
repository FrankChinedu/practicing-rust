pub struct Solution;

fn binary_to_decimal(binary: &str) -> u32 {
    let mut decimal = 0;
    for (i, c) in binary.chars().rev().enumerate() {
        let digit = c.to_digit(2).expect("Invalid binary string");
        decimal += digit * 2_u32.pow(i as u32);
    }
    decimal
}

fn convert_to_binary(val: u32) -> String {
    let mut remainder = "".to_string();
    let mut current_val = val as f64;

    loop {
        if current_val == 1.0 {
            remainder = format!("{current_val}{remainder}");
            break;
        }
        let modulus = (current_val % 2.0) as i32;
        let val = (current_val / 2.0).floor();
        remainder = format!("{modulus}{remainder}");
        current_val = val
    }
    remainder
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = binary_to_decimal(&a);
        let b = binary_to_decimal(&b);
        let ans = a + b;
        convert_to_binary(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "100");
    }

    #[test]
    fn simple_column_number_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "10101");
    }
}
