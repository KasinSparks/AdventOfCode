use crate::utils::io::{read_file_split_newlines};

pub fn sln(input_path: &str) -> i32 {
    let input: Vec<String> = read_file_split_newlines(input_path);
    let mut result: i32 = 0;

    for line in input {
        let mut first_digit: i32 = 0;
        let mut last_digit: i32= 0;

        let line_bytes = line.as_bytes();

        // Find the first digit
        for i in 0..line_bytes.len() {
            if line_bytes[i].is_ascii_digit() {
                first_digit = (line_bytes[i] - ('0' as u8)) as i32;
                break;
            }
        }

        // Find the last digit
        for i in (0..line_bytes.len()).rev() {
            if line_bytes[i].is_ascii_digit() {
                last_digit = (line_bytes[i] - ('0' as u8)) as i32;
                break;
            }
        }
        
        // Concat and convert to integer
        result += (first_digit * 10) + last_digit;
        println!("LINE: {}, {}, {}", line, first_digit, last_digit);
    }
    

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_01/practice_input.txt"), 142);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_01/input.txt"), 54632);
    }
}