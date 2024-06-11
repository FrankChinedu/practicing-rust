pub struct Solution;

fn slice_to_vec(nums: &mut [i32]) -> Vec<i32> {
    nums.to_vec()
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let slice_num1 = &mut nums1[0..m as usize];
        let num_1 = slice_to_vec(slice_num1);
        nums1.clear();
        nums1.extend(num_1);
        let slice_num2 = &mut nums2[0..n as usize];
        let num_2 = slice_to_vec(slice_num2);
        nums2.clear();
        nums2.extend(num_2);
        nums1.extend(std::mem::take(nums2));
        nums1.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut num1 = Vec::from([1, 2, 3, 0, 0, 0]);
        let mut num2 = Vec::from([2, 5, 6]);
        let m = 3;
        let n = 3;
        Solution::merge(&mut num1, m, &mut num2, n);
        let result = Vec::from([1, 2, 2, 3, 5, 6]);
        assert_eq!(result, num1);
    }

    #[test]
    fn test_2() {
        let mut num1 = Vec::from([1]);
        let mut num2 = Vec::from([]);
        let m = 1;
        let n = 0;
        Solution::merge(&mut num1, m, &mut num2, n);
        let result = Vec::from([1]);
        assert_eq!(result, num1);
    }

    #[test]
    fn test_3() {
        let mut num1 = Vec::from([0]);
        let mut num2 = Vec::from([1]);
        let m = 0;
        let n = 1;
        Solution::merge(&mut num1, m, &mut num2, n);
        let result = Vec::from([1]);
        assert_eq!(result, num1);
    }

    #[test]
    fn test_4() {
        let mut num1 = Vec::from([-1, 0, 0, 3, 3, 3, 0, 0, 0]);
        let mut num2 = Vec::from([1, 2, 2]);
        let m = 6;
        let n = 3;
        Solution::merge(&mut num1, m, &mut num2, n);
        let result = Vec::from([-1, 0, 0, 1, 2, 2, 3, 3, 3]);
        assert_eq!(result, num1);
    }
}
