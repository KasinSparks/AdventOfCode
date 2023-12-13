use crate::utils::{io::read_file_split_newlines, grid_2d::Grid2D};

pub fn sln(input_path: &str) -> u64 {
    // Note: input must end with an empty new line
    let input = read_file_split_newlines(input_path);

    let mut map_ranges = Vec::new();

    let mut curr_start = 0;
    for (i, line) in input.iter().enumerate() {
        if line == "" {
            map_ranges.push((curr_start, i));
            curr_start = i + 1;
        }
    }

    let maps: Vec<&[String]> = map_ranges.iter()
                         .map(|range| &input[range.0..range.1])
                         .collect();

    let mut result = 0;
    for m in maps {
        if let Some(temp) = find_reflection(m) {
            println!("test: {}", temp);
            result += temp as u64;
        } else {
            println!("non-mirrored map: {:#?}", m);
        }
    }


    return result;
}

fn find_reflection(map: &[String]) -> Option<u32> {
    let length = map.len();
    let width = map[0].len();

    let mut mid_col = Vec::new();
    let mut mid_row = Vec::new();

    for r in 0..length - 1 {
        if check_rows(map, (r, r+1)) {
            mid_row.push(r);
        }
    }

    for c in 0..width - 1 {
        if check_cols(map, (c, c+1)) {
            mid_col.push(c);
        }
    }



    let mut trackers = (0, 0);

    for mr in mid_row {
        if mr != 0 && check_rows(map, (mr, mr - 1)) {
            // up
            let mid_point = mr + 1;
            let mut is_mirrored = true;

            if mr == length - 1 {
                return Some(mid_point as u32 * 100);
            }

            trackers = (mr + 1, mr - 2);
            while trackers.0 < length && trackers.1 >= 0 {
                if !check_rows(map, trackers) {
                    is_mirrored = false;
                    break;
                }

                if trackers.1 == 0 {
                    break;
                }

                trackers = (trackers.0 + 1, trackers.1 - 1);
            }

            if is_mirrored {
                return Some(mid_point as u32 * 100);
            }        
        } 

        if mr != length - 1 && check_rows(map, (mr, mr + 1)) {
            // down
            let mid_point = mr + 1;
            let mut is_mirrored = true;

            if mr == 0 {
                return Some(mid_point as u32 * 100);
            }

            trackers = (mr + 2, mr - 1);
            while trackers.0 < length && trackers.1 >= 0 {
                if !check_rows(map, trackers) {
                    is_mirrored = false;
                    break;
                }

                if trackers.1 == 0 {
                    break;
                }

                trackers = (trackers.0 + 1, trackers.1 - 1);

            }

            if is_mirrored {
                return Some(mid_point as u32 * 100);
            }        
        }
    }

    for mc in mid_col { 
        if mc != 0 && check_cols(map, (mc, mc - 1)) {
            // left
            let mid_point = mc + 1;
            let mut is_mirrored = true;

            if mc == width - 1 {
                return Some(mid_point as u32);
            }

            trackers = (mc + 1, mc - 2);
            while trackers.0 < width && trackers.1 >= 0 {
                if !check_cols(map, trackers) {
                    is_mirrored = false;
                    break;
                }

                if trackers.1 == 0 {
                    break;
                }

                trackers = (trackers.0 + 1, trackers.1 - 1);
            }

            if is_mirrored {
                return Some(mid_point as u32);
            }        
        } 
        
        if mc != width - 1 && check_cols(map, (mc, mc + 1)) {
            // right
            let mid_point = mc + 1;
            let mut is_mirrored = true;

            if mc == 0 {
                return Some(mid_point as u32);
            }

            trackers = (mc + 2, mc - 1);
            while trackers.0 < width && trackers.1 >= 0 {
                if !check_cols(map, trackers) {
                    is_mirrored = false;
                    break;
                }

                if trackers.1 == 0 {
                    break;
                }

                trackers = (trackers.0 + 1, trackers.1 - 1);

            }

            if is_mirrored {
                return Some(mid_point as u32);
            }        
        }
    }

    // no reflection
    return None;
}

fn check_rows(map: &[String], rows: (usize, usize)) -> bool {
    if map[rows.0] == map[rows.1] {
        return true;
    }

    return false;
}

fn check_cols(map: &[String], cols: (usize, usize)) -> bool {
    let mut result = true;
    for row in map {
        let row_data = row.as_bytes();
        if row_data[cols.0] != row_data[cols.1] {
            result = false;
            break;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_13/practice_input.txt"), 405);
    }

    // 35520 too low
    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_13/input.txt"), 36015);
    }
}