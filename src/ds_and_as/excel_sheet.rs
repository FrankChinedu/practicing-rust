pub struct Solution;

fn get_number_array(column_number: usize, alpahbet_size: usize) -> Vec<usize> {
    let mut arr_num = vec![];

    if column_number <= alpahbet_size {
        return vec![column_number];
    }

    let alpahbet_size = alpahbet_size as f64;
    let mut column_number = column_number as f64;

    loop {
        let modulus = (column_number % alpahbet_size) as usize;
        if alpahbet_size >= column_number {
            arr_num.insert(0, column_number as usize);
            break;
        }
        arr_num.insert(0, modulus);
        column_number = (column_number / alpahbet_size).floor();
    }
    arr_num
}

fn get_alphabet(alphabet: Vec<char>, arr_of_nums: Vec<usize>) -> String {
    let mut arr_of_char = vec![];

    for num in arr_of_nums {
        let index = num - 1;
        let d = alphabet.get(index).unwrap();
        let output = format!("{}", d);
        arr_of_char.push(output);
    }
    arr_of_char.join("")
}

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let column_number = column_number as usize;
        let mut alphabet = Vec::new();

        for ascii_value in 97..=122 {
            let alp = (ascii_value as u8) as char;
            alphabet.push(alp.to_ascii_uppercase());
        }
        let size_of_alphabet = alphabet.len();

        let arr = get_number_array(column_number, size_of_alphabet);

        get_alphabet(alphabet, arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_column_number_1() {
        let column_number = 1;
        let result = Solution::convert_to_title(column_number);
        let val = String::from("A");
        assert_eq!(result, val);
    }

    #[test]
    fn simple_column_number_2() {
        let column_number = 25;
        let result = Solution::convert_to_title(column_number);
        let val = String::from("Y");
        assert_eq!(result, val);
    }

    #[test]
    fn simple_column_number_3() {
        let column_number = 26;
        let result = Solution::convert_to_title(column_number);
        let val = String::from("Z");
        assert_eq!(result, val);
    }

    #[test]
    fn simple_column_number_for_4() {
        let column_number = 28;
        let result = Solution::convert_to_title(column_number);
        let val = String::from("AB");
        assert_eq!(result, val);
    }

    #[test]
    fn simple_column_number_for_5() {
        let column_number = 701;
        let result = Solution::convert_to_title(column_number);
        let val = String::from("ZY");
        assert_eq!(result, val);
    }
}
