pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle = &needle;
        let haystack = &haystack;
        let needle_length = needle.len();
        let mut count = 0;

        loop {
            let num = needle_length + count;
            if num > haystack.len() {
                break -1;
            }
            let str_slice = &haystack[count..num];
            if str_slice.len() > needle_length {
                break -1;
            }
            if str_slice == needle {
                break count as i32;
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, 0);
    }

    #[test]
    fn simple_column_number_2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");
        let result = Solution::str_str(haystack, needle);
        assert_eq!(result, -1);
    }
}
