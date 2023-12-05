use crate::utils::io::{read_file};

pub fn sln(input_path: &str) -> i32 {
    let input: String = read_file(input_path);

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_00/practice_input.txt"), 0);
    }

    /*
    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_00/input.txt"), 0);
    }
    */
}