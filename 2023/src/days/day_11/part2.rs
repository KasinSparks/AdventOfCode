use crate::utils::{io::read_file_split_newlines, grid_2d::Point};

pub fn sln(input_path: &str, expand_amt: usize) -> i64 {
    //const EXPAND_SIZE: usize = 1_000_000;
    //const EXPAND_SIZE: usize = 2;
    let input = read_file_split_newlines(input_path);

    let expanded = expander(&input);

    let mut points: Vec<(usize, Point<usize>)> = Vec::new();
    let mut total_steps = 0;

    let mut counter = 1;
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.char_indices() {
            if c != '.' {
                points.push((counter, Point { x: j, y: i }));
                counter += 1;
            }

        }
    }

    //println!("Points: {:?}", points);


    for (i, ref_p) in points.iter().enumerate() {
        let mut ref_p_modifer: (usize, usize) = (0, 0);

        for m in &expanded.0 {
            if ref_p.1.y > *m {
                ref_p_modifer.1 += expand_amt - 1;
            }
        }
        for m in &expanded.1 {
            if ref_p.1.x > *m {
                ref_p_modifer.0 += expand_amt - 1;
            }
        }

        for target_p in &points[i+1..points.len()] {
            let mut target_p_modifer: (usize, usize) = (0, 0);
            for m in &expanded.0 {
                if target_p.1.y > *m {
                    target_p_modifer.1 += expand_amt - 1;
                }
            }
            for m in &expanded.1 {
                if target_p.1.x > *m {
                    target_p_modifer.0 += expand_amt - 1;
                }
            }

            let mut s = i64::abs((ref_p_modifer.0 + ref_p.1.x) as i64 - (target_p.1.x + target_p_modifer.0) as i64);
            s += i64::abs((ref_p_modifer.1 + ref_p.1.y) as i64 - (target_p.1.y + target_p_modifer.1) as i64);
            total_steps += s;
            //println!("{} -> {}, rpm: {:?}, ref_p: {:?}, tpm: {:?}, target_p: {:?}, steps: {}", ref_p.0, target_p.0, ref_p_modifer, ref_p.1, target_p_modifer, target_p.1, s);
        }
    }

    return total_steps;
}

fn expander(input: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    let mut expand_pos: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());

    // Horizontal
    for (i, line) in input.iter().enumerate() {
        let mut is_empty = true;
        for c in line.chars() {
            if c != '.' {
                is_empty = false;
                break;
            }
        } 

        //new_vec.push(line.to_string());

        if is_empty {
            expand_pos.0.push(i);
        }
    }

    // Vertical
    for i in 0..input.get(0).unwrap().len() {
        let mut is_empty = true;

        for j in 0..input.len() {
            let c = input.get(j).unwrap().as_bytes()[i] as char;
            //vertical_line.push(c);
            if c != '.' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            expand_pos.1.push(i);
        }
    }


    return expand_pos;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expander_test() {
        let expected = (vec![3, 7], vec![2, 5, 8]);
        let expaned = expander(&read_file_split_newlines("./src/days/day_11/practice_input.txt"));
        assert_eq!(expected, expaned);
    }

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_11/practice_input.txt", 2), 374);
    }

    #[test]
    fn practice_result2() {
        assert_eq!(sln("./src/days/day_11/practice_input.txt", 10), 1030);
    }

    #[test]
    fn practice_result3() {
        assert_eq!(sln("./src/days/day_11/practice_input.txt", 100), 8410);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_11/input.txt", 1_000_000), 752936133304);
    }
}