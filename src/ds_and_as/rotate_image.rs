use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut hash_set: HashSet<(usize, usize)> = HashSet::new();
        let row_len = matrix.len();
        let col_len = matrix[0].len();
        for x in 0..row_len {
            for y in 0..col_len {
                let out_code = (x, y);
                if hash_set.contains(&out_code) {
                    continue;
                }

                let mut x_copy = x;
                let mut y_copy = y;
                let mut current = matrix[x_copy][y_copy];
                for _ in 0..4 {
                    let cood = next_coodinate(x_copy, y_copy, row_len);
                    std::mem::swap(&mut matrix[cood.0][cood.1], &mut current);
                    x_copy = cood.0;
                    y_copy = cood.1;
                    hash_set.insert(cood);
                }
            }
        }
    }
}

fn next_coodinate(x: usize, y: usize, matrix_len: usize) -> (usize, usize) {
    let x = (matrix_len - 1) - x;
    (y, x)
}

// fn swap(matrix: &mut Vec<Vec<i32>>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = Vec::from([
            Vec::from([1, 2, 3]),
            Vec::from([4, 5, 6]),
            Vec::from([7, 8, 9]),
        ]);
        Solution::rotate(&mut matrix);
        let result = Vec::from([
            Vec::from([7, 4, 1]),
            Vec::from([8, 5, 2]),
            Vec::from([9, 6, 3]),
        ]);
        assert_eq!(result, matrix);
    }
    #[test]
    fn test_2() {
        let mut matrix = Vec::from([
            Vec::from([5, 1, 9, 11]),
            Vec::from([2, 4, 8, 10]),
            Vec::from([13, 3, 6, 7]),
            Vec::from([15, 14, 12, 16]),
        ]);
        Solution::rotate(&mut matrix);
        let result = Vec::from([
            Vec::from([15, 13, 2, 5]),
            Vec::from([14, 3, 4, 1]),
            Vec::from([12, 6, 8, 9]),
            Vec::from([16, 7, 10, 11]),
        ]);
        assert_eq!(result, matrix);
    }
}
