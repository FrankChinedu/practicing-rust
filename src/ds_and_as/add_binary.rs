pub struct Solution;

fn _binary_to_decimal(binary: &str) -> u32 {
    let mut decimal = 0;
    for (i, c) in binary.chars().rev().enumerate() {
        let digit = c.to_digit(2).expect("Invalid binary string");
        decimal += digit * 2_u32.pow(i as u32);
    }
    decimal
}

fn _convert_to_binary(val: u32) -> String {
    let mut remainder = "".to_string();
    let mut current_val = val as f64;

    loop {
        if current_val == 1.0 {
            remainder = format!("{current_val}{remainder}");
            break;
        }
        let modulus = (current_val % 2.0) as i32;
        let val = (current_val / 2.0).floor();
        remainder = format!("{modulus}{remainder}");
        current_val = val
    }
    remainder
}

fn add(val: Vec<String>) -> Vec<String> {
    let val = val
        .iter()
        .map(|x| u128::from_str_radix(x, 2).expect("Invalid binary number"))
        .collect::<Vec<_>>();
    let ans = val.iter().sum::<u128>();
    let mut val = format!("{:b}", ans);
    if val.len() == 1 {
        val = format!("0{val}");
    }
    val.split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect::<Vec<_>>()
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut big = "".to_string();
        let mut small = "".to_string();
        if a.len() > b.len() {
            big = a;
            small = b;
        } else {
            big = b;
            small = a;
        }

        let mut num = "0".to_string();
        let mut item = "".to_string();
        let a = big;
        let b = small;
        println!("a {a}");
        println!("b {b}");
        let b = b.chars().rev().collect::<String>();

        for (i, val) in a.chars().rev().enumerate() {
            let temp_num = num.clone().to_string();
            let mut vec = vec![val.to_string(), temp_num];
            match b.chars().nth(i) {
                Some(x) => {
                    let x = x.to_string();
                    vec.push(x);
                    let res = add(vec);
                    let last = res.last().unwrap().to_string();
                    let first = res.first().unwrap().to_string();
                    item = format!("{item}{last}");
                    num = first;
                }
                None => {
                    let res = add(vec);
                    let last = res.last().unwrap().to_string();
                    let first = res.first().unwrap().to_string();
                    item = format!("{item}{last}");
                    num = first;
                }
            }
        }
        println!("item {item}");
        item = item.chars().rev().collect::<String>();
        if num == *"1" {
            item = format!("{num}{item}");
        }
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "100");
    }

    #[test]
    fn simple_column_number_2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, "10101");
    }
}
