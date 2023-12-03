use crate::utils::io::read_file_split_newlines;

pub fn sln(input_path: &str) -> i32 {
    let input: Vec<String> = read_file_split_newlines(input_path);
    
    if input.is_empty() {
        return 0;
    }

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input {
        let mut row: Vec<char> = Vec::new();

        for c in line.chars() {
            row.push(c);
        }

        grid.push(row);
    }


    // Find the symbols
    let mut symbols: Vec<(i32, i32)> = Vec::new();

    //for row in grid {
        //for cell in row {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let cell = grid[row][col];
            match cell {
                '*' | '#' | '+' | '$' | '!' | '@' | '%' | '&' | '(' | ')' | '`' | '~' | '-' | '_' | '=' | 
                '[' | ']' | '{' | '}' | ';' | ':' | '\"' | '\'' | ',' | '/' | '?' | '<' | '>' | 
                '\\' | '|' => {
                    symbols.push((row as i32, col as i32));
                } 
                _ => {}
            };
        }
    }

    let mut digits: Vec<(i32, i32)> = Vec::new();

    let neighbors_grid: [(i32, i32); 9] = [
        (-1, 1),  (0, 1),  (1, 1),
        (-1, 0),  (0, 0),  (1, 0),
        (-1, -1), (0, -1), (1, -1),
    ];

    // Search the neighbors for digits
    for s in symbols {
        for n in neighbors_grid {
            let r: (i32, i32) = (n.0 + s.0, n.1 + s.1);

            if (r.0 >= 0 && r.0 < grid.len() as i32) && (r.1 >= 0 && r.1 < grid[r.0 as usize].len() as i32) {
                //println!("checking {:?}", r);
                let d = grid[r.0 as usize][r.1 as usize];
                if d.is_ascii_digit() {
                    digits.push(r);
                }
            }
        }
    }

    println!("digits: {:?}", digits);

    let mut results: Vec<i32> = Vec::new();

    let mut has_been_checked: Vec<Vec<char>> = grid.clone();

    for r in 0..has_been_checked.len() {
        for c in 0..has_been_checked[r].len() {
            has_been_checked[r][c] = '\0';
        }
    }

    // Get the whole number (left & right)
    for d in digits {
        let mut dig = String::from("");

        if has_been_checked[d.0 as usize][d.1 as usize] != '\0' {
            // Already has been checked
            continue;
        }

        // Check left
        let mut cc: i32 = d.1 - 1;
        while (cc >= 0 && cc < grid[d.0 as usize].len() as i32) && grid[d.0 as usize][cc as usize].is_ascii_digit() 
            && has_been_checked[d.0 as usize][cc as usize] == '\0' {

            dig.push(grid[d.0 as usize][cc as usize]);
            has_been_checked[d.0 as usize][cc as usize] = '1';
            cc -= 1;
        }

        dig = dig.chars().rev().collect();

        // Check right
        let mut cc: i32 = d.1;
        while (cc >= 0 && cc < grid[d.0 as usize].len() as i32) && grid[d.0 as usize][cc as usize].is_ascii_digit() 
            && has_been_checked[d.0 as usize][cc as usize] == '\0' {

            dig.push(grid[d.0 as usize][cc as usize]);
            has_been_checked[d.0 as usize][cc as usize] = '1';
            cc += 1;
        }


        // save the result
        results.push(bytes_to_num(dig.as_bytes()) as i32);
    }

    println!("Results: {:?}", results);

    let mut result = 0;
    for r in results {
        result += r;
    }

    return result;
}

/// TODO: move this to utils
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
        assert_eq!(sln("./src/days/day_03/practice_input.txt"), 4361);
    }
}