use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut hash: HashMap<i32, i32> = HashMap::new();

        fn get_fib(key: i32, hash: &mut HashMap<i32, i32>) -> i32 {
            if key == 0 {
                0
            } else if key == 1 {
                return 1;
            } else if key == 2 {
                return 2;
            } else if key == 3 {
                return 3;
            } else {
                match hash.get(&key) {
                    Some(val) => return *val,
                    None => {
                        let val = get_fib(key - 1, hash) + get_fib(key - 2, hash);
                        hash.insert(key, val);
                        val
                    }
                }
            }
        }
        get_fib(n, &mut hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let x = 2;
        let result = Solution::climb_stairs(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn simple_column_number_2() {
        let x = 3;
        let result = Solution::climb_stairs(x);
        assert_eq!(result, 3);
    }

    #[test]
    fn simple_column_number_3() {
        let x = 4;
        let result = Solution::climb_stairs(x);
        assert_eq!(result, 5);
    }
}
