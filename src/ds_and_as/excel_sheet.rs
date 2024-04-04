pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let column_number = column_number as usize;
        let mut alphabet = Vec::new();

        for ascii_value in 97..=122 {
            let alp = (ascii_value as u8) as char;
            alphabet.push(alp.to_ascii_uppercase());
        }
        let size_of_alphabet = alphabet.len();

        if size_of_alphabet >= column_number {
            let index = column_number - 1;
            let d = alphabet.get(index).unwrap();
            let output = format!("{}", d);
            return output;
        }

        let index = (column_number as f64 / size_of_alphabet as f64).floor() as usize - 1;

        // let
        let char = alphabet.get(index).unwrap();
        let char = format!("{}", char);
        let mut alp_arr = vec![];
        alp_arr.push(char);

        let index = (column_number % size_of_alphabet) - 1;
        let char = alphabet.get(index).unwrap();
        let char = format!("{}", char);
        alp_arr.push(char);

        alp_arr.join("")
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
