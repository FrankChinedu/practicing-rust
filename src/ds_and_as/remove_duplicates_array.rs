use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut map: HashSet<i32> = HashSet::new();
        let k = nums
            .iter()
            .filter(|x| {
                if !map.contains(x) {
                    map.insert(**x);
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();

        k.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = Vec::from([1, 1, 2]);
        let result = Solution::remove_duplicates(&mut nums);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let mut nums = Vec::from([0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
        let result = Solution::remove_duplicates(&mut nums);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
