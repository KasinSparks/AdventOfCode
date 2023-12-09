use crate::utils::io::read_file_split_newlines;

pub fn sln(input_path: &str) -> i64 {
    let input = read_file_split_newlines(input_path);

    let mut result = 0;

    for line in input {
        let nums: Vec<i64> = line.split(' ')
                                 .map(|num_str| num_str.parse::<i64>().unwrap())
                                 .collect();

        //println!("nums: {:?}", nums);

        //println!("val: {}", nums.last().unwrap() + diff_loop(&nums));
        result += nums.last().unwrap() + diff_loop(&nums);
    }

    return result;
}

fn diff_loop(vals: &Vec<i64>) -> i64 {
    let mut new_vals = Vec::new();

    for i in 0..vals.len() - 1 {
        new_vals.push(vals[i + 1] - vals[i]);
    }

    let mut is_all_zeros = true;
    for val in &new_vals {
        if *val != 0 {
            is_all_zeros = false;
            break;
        }
    }

    //println!("newval: {:?}", new_vals);

    if !is_all_zeros {
        return *new_vals.last().unwrap() + diff_loop(&new_vals);
    }

    return *new_vals.last().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_09/practice_input.txt"), 114);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_09/input.txt"), 1725987467);
    }
}