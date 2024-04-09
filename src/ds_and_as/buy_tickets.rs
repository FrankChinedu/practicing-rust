pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut tickets = tickets;
        let mut time = 0;
        'outet: loop {
            for index in 0..tickets.len() {
                if tickets[index] == 0 {
                    continue;
                }
                time += 1;
                let val = tickets[index] - 1;

                tickets[index] = val;
                if k as usize == index && val == 0 {
                    break 'outet;
                };
            }
            println!("{:?}", tickets);
        }
        time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let tickets = vec![2, 3, 2];
        let result = Solution::time_required_to_buy(tickets, 2);
        assert_eq!(result, 6);
    }

    #[test]
    fn simple_column_number_2() {
        let tickets = vec![5, 1, 1, 1];
        let result = Solution::time_required_to_buy(tickets, 0);
        assert_eq!(result, 8);
    }
}
