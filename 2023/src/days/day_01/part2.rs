use crate::utils::io::{read_file_split_newlines};

// Note: I got stuck on the wording for this part.
// ex: oneight should be one and eight...

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hit {
    pos: usize,
    val: u8,
}

pub fn sln(input_path: &str) -> i32 {
    let input: Vec<String> = read_file_split_newlines(input_path);
    let mut result: i32 = 0;

    for line in input {
        let mut tokens: Vec<Hit> = Vec::new();
        let word_nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        for i in 1..=word_nums.len(){
            if let Some(pos) = line.find(word_nums[i-1]) {
                tokens.push(Hit { pos, val: i as u8});
            }
            if let Some(pos) = line.rfind(word_nums[i-1]) {
                tokens.push(Hit { pos, val: i as u8});
            }
        }

        for i in 0..=9 {
            if let Some(pos) = line.find(i.to_string().as_str()) {
                tokens.push(Hit { pos, val: i});
            }
            if let Some(pos) = line.rfind(i.to_string().as_str()) {
                tokens.push(Hit { pos, val: i});
            }
        }

        tokens.sort();

        let first_digit: u8 = tokens.get(0).unwrap().val;
        let last_digit: u8 = tokens.get(tokens.len() - 1).unwrap().val;

        // Concat and convert to integer
        result += ((first_digit * 10) + last_digit) as i32;
        println!("LINE: {}, {}, {}", line, first_digit, last_digit);
    }
    

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_01/practice_input2.txt"), 281);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_01/input.txt"), 54019);
    }
}