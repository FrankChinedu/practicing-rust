pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        println!("{:?}", strs);
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let strs = Vec::from(["flower".to_owned(), "flow".to_owned(), "flight".to_owned()]);
        let result = Solution::longest_common_prefix(strs);
        let expected = "fl";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let strs = Vec::from(["dog".to_owned(), "racecar".to_owned(), "car".to_owned()]);
        let result = Solution::longest_common_prefix(strs);
        let expected = "";
        assert_eq!(result, expected);
    }
}
