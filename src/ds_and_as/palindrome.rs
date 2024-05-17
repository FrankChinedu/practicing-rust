pub struct Solution;

impl Solution {
    pub fn is_palindrome(num: i32) -> bool {
        let og_string = format!("{}", num);
        let reverse = og_string.chars().rev().collect::<String>();
        if og_string == reverse {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let num = 121;
        let result = Solution::is_palindrome(num);
        assert!(result);
    }

    #[test]
    fn it_works_2() {
        let num = -121;
        let result = Solution::is_palindrome(num);
        assert!(!result);
    }
}
