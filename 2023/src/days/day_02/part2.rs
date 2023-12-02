use crate::utils::io::{read_file, read_file_with_split, read_file_split_newlines};

#[derive(Debug)]
struct SetData {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug)]
struct GameData {
    id: u32,
    data: Vec<SetData>,
}

pub fn sln(input_path: &str) -> i32 {
    let input: Vec<String> = read_file_split_newlines(input_path);
    let mut game_data: Vec<GameData> = Vec::new(); 



    for line in input {
        if let Some(result) = parser(line.as_str()) {
            game_data.push(result);
        }
    }

    
    // Total
    let mut total: i32 = 0;
    for gd in game_data {
        let mut max_sd = SetData {blue: 0, red: 0, green: 0};

        for sd in &gd.data {
            if sd.blue > max_sd.blue {
                max_sd.blue = sd.blue;
            }

            if sd.red > max_sd.red {
                max_sd.red = sd.red;
            }

            if sd.green > max_sd.green {
                max_sd.green = sd.green;
            }
        }

        let power = max_sd.blue * max_sd.red * max_sd.green;
        println!("Power: {}, {:?}", power, gd);
        total += power as i32;
    }

    return total;
}

fn parser(line: &str) -> Option<GameData> {
    let mut gd: GameData = GameData {
        id: 0,
        data: Vec::new()
    };

    let mut curr_pos = 5;
    let line_data = line.as_bytes();

    // Parse Game ###:
    if curr_pos >= line_data.len() {
        return None;
    }

    while line_data[curr_pos].is_ascii_digit() {
        curr_pos += 1;
    }

    // Record the game id
    let game_id_str = &(line_data[5..curr_pos]);
    gd.id = bytes_to_num(game_id_str);

    // Skip the :
    curr_pos += 1;

    // Parse the sets
    let mut sets: Vec<String> = Vec::new();
    let mut curr_set: String = String::from("");

    while curr_pos < line_data.len() {
        // Go until we hit a semicolon for each set
        if line_data[curr_pos] == ';' as u8 {
            // Go to next set
            sets.push(curr_set.trim().to_string());
            curr_set.clear();
            curr_pos += 1;
            continue;
            //curr_set = String::from("");
        }

        curr_set.push(line_data[curr_pos] as char);
        curr_pos += 1;
    }

    // Push the final set
    sets.push(curr_set.trim().to_string());

    // Go through the sets
    let mut sub_sets: Vec<SetData> = Vec::new();

    for set in sets {
        //println!("Set: {}", set);
        // Split on commas
        let set_data: Vec<String> = set.split(',').map(String::from).collect();
        let mut val: SetData = SetData { blue: 0, red: 0, green: 0 };
        for sd in set_data {
            //println!("Set data: {}", sd);
            // Get number and type
            let d: Vec<String> = sd.trim().split(' ').map(String::from).collect();
            let n = bytes_to_num(d[0].trim().as_bytes());
            match d[1].trim() {
                "blue" => {val.blue = n;},
                "green" => {val.green = n;},
                "red" => {val.red = n;},
                _ => { println!("Unknown color type: {}", d[1].trim()); return None; }
            };
        }
        sub_sets.push(val);
    }

    //let mut sd: SetData = SetData { blue: 0, red: 0, green: 0 };

    for ss in sub_sets {
        gd.data.push(ss);
    }

    //gd.data = sd;
    println!("game data: {:?}", gd);

    return Some(gd);

}

fn bytes_to_num(nums: &[u8]) -> u32 {
    let l = nums.len();
    let mut result = 0;

    for i in (0..l).rev() {
        result += u32::pow(10, i as u32) * (nums[(l - 1) - i] - '0' as u8) as u32;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_02/practice_input.txt"), 2286);
    }
}