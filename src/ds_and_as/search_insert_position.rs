pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let length = nums.len();
        let half_of_length = ((length / 2) as f64).floor() as usize;
        let left_of_vec = half_of_length;
        if length == 0 {
            return 0;
        }

        if length == 1 {
            if target <= nums[0] {
                return 0;
            }
            if target > nums[0] {
                return 1;
            }
        }

        let left_vec = &nums[0..left_of_vec];
        let right_vec = &nums[left_of_vec..length];

        let last_item = left_vec[half_of_length - 1];

        if target <= last_item || target < right_vec[0] {
            let mut val = 0;
            for (i, num) in left_vec.iter().enumerate() {
                if target == *num {
                    val = i as i32;
                    break;
                }
                if *num > target {
                    val = i as i32;
                    break;
                }
                if target > *num {
                    val = i as i32 + 1;
                }
            }
            return val;
        }
        let mut val = length as i32;

        for (i, num) in right_vec.iter().enumerate() {
            if target == *num {
                val = i as i32 + left_of_vec as i32;
                break;
            }
            if target < *num {
                val = i as i32 + left_of_vec as i32;
                break;
            }
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1b() {
        let nums = vec![1];
        let target = 0;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 0);
    }

    #[test]
    fn simple_column_number_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn simple_column_number_1a() {
        let nums = vec![1, 3, 6, 7];
        let target = 5;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn simple_column_number_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 1);
    }

    #[test]
    fn simple_column_number_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let result = Solution::search_insert(nums, target);
        assert_eq!(result, 4);
    }
}
