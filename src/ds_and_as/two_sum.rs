pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vec: Vec<i32> = vec![];

        'first: for (i, num) in nums.iter().enumerate() {
            if i + 1 == nums.len() {
                break;
            };

            for (j, other) in nums.iter().enumerate() {
                if i == j {
                    continue;
                }
                let res = num + other;
                if res == target {
                    vec.push(i as i32);
                    vec.push((j) as i32);
                    break 'first;
                }
            }
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);

        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn it_works_4() {
        let nums = vec![2, 11, 7, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);

        assert_eq!(result, [0, 2]);
    }

    #[test]
    fn it_works_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);

        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn it_works_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);

        assert_eq!(result, [0, 1]);
    }
}
