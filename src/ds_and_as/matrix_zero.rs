pub struct Solution;

#[derive(Debug)]
enum Direction {
    Up { x: i32, y: i32 },
    Down { x: i32, y: i32 },
    Left { x: i32, y: i32 },
    Right { x: i32, y: i32 },
}

const UP: Direction = Direction::Up { x: -1, y: 0 };
const DOWN: Direction = Direction::Down { x: 1, y: 0 };
const LEFT: Direction = Direction::Left { x: 0, y: -1 };
const RIGHT: Direction = Direction::Right { x: 0, y: 1 };

fn is_negative(num: &i32) -> bool {
    num < &0
}

fn at_egde(val: usize, len: usize) -> bool {
    val >= len
}

fn move_mat(dir: &Direction, tup: (i32, i32), matrix: &mut Vec<Vec<i32>>) {
    let mut tup_x = tup.0;
    let mut tup_y = tup.1;
    match dir {
        Direction::Up { x, y } => {
            tup_x += x;
            tup_y += y;
        }
        Direction::Down { x, y } => {
            tup_x += x;
            tup_y += y;
        }
        Direction::Left { x, y } => {
            tup_x += x;
            tup_y += y;
        }
        Direction::Right { x, y } => {
            tup_x += x;
            tup_y += y;
        }
    };
    if is_negative(&tup_x) || is_negative(&tup_y) {
        return;
    }
    let x_len = matrix.len();
    let y_len = matrix[0].len();
    if at_egde(tup_x as usize, x_len) || at_egde(tup_y as usize, y_len) {
        return;
    }
    let tup = (tup_x, tup_y);
    matrix[tup.0 as usize][tup.1 as usize] = 0;
    move_mat(dir, tup, matrix)
}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let directions = [UP, DOWN, LEFT, RIGHT];
        let mut vec: Vec<(usize, usize)> = vec![];
        for (i, item) in matrix.iter().enumerate() {
            for (j, val) in item.iter().enumerate() {
                if val == &0 {
                    vec.push((i, j))
                }
            }
        }

        for tuple in vec {
            for dir in &directions {
                let tup = (tuple.0 as i32, tuple.1 as i32);
                move_mat(dir, tup, matrix)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = Vec::from([
            Vec::from([1, 1, 1]),
            Vec::from([1, 0, 1]),
            Vec::from([1, 1, 1]),
        ]);
        Solution::set_zeroes(&mut matrix);
        let result = Vec::from([
            Vec::from([1, 0, 1]),
            Vec::from([0, 0, 0]),
            Vec::from([1, 0, 1]),
        ]);
        assert_eq!(result, matrix);
    }
    #[test]
    fn test_2() {
        let mut matrix = Vec::from([
            Vec::from([0, 1, 2, 0]),
            Vec::from([3, 4, 5, 2]),
            Vec::from([1, 3, 1, 5]),
        ]);
        Solution::set_zeroes(&mut matrix);
        let result = Vec::from([
            Vec::from([0, 0, 0, 0]),
            Vec::from([0, 4, 5, 0]),
            Vec::from([0, 3, 1, 0]),
        ]);
        assert_eq!(result, matrix);
    }
}
