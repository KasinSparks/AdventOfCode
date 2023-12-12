use crate::utils::io::read_file_split_newlines;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpringCondition {
    Damaged,
    Functional,
    Unknown
}

impl SpringCondition {
    pub fn from(c: &char) -> SpringCondition {
        match c {
            '#' => Self::Functional,
            '.' => Self::Damaged,
            '?' => Self::Unknown,
            _ => Self::Unknown 
        }
    }
}

#[derive(Debug)]
struct SpringArrangement {
    arrangement: Vec<SpringCondition>
}

#[derive(Debug)]
struct SpringRecord {
    data: Vec<SpringCondition>,
    info: Vec<u8>
}

impl SpringRecord {
    pub fn new(line: &String) -> Self {
        let record_info: Vec<&str> = line.split(' ').collect();

        let d = record_info[0].chars()
                                                    .enumerate()
                                                    .map(|(i, c)| SpringCondition::from(&c))
                                                    .collect();

        let i = record_info[1].split(',')
                                       .map(|c| c.parse().unwrap())
                                       .collect();

        return Self { data: d, info: i };
    }

    /*
    pub fn get_num_of_arrangements(&self) -> u64 {
        let mut min_spaces_needed = self.info.len() as u8 * self.info.iter().sum::<u8>();
        return min_spaces_needed as u64;
    }
    */

    pub fn get_num_of_arrangements(&self) -> u32 {
        let mut count = 0;
        for p in self.get_arrangement_permutations() {
            match self.try_arrangement(p) {
                Ok(b) => {
                    if b {
                        count += 1
                    }
                },
                Err(msg) => {}
            }
        }

        return count;
    }

    fn get_arrangement_permutations(&self) -> Vec<SpringArrangement> {
        let mut arrangements = Vec::new();

        let binary_rep: Vec<u8> = self.data.iter()
                                  .map(
                                    |sc| 
                                    match sc {
                                        SpringCondition::Unknown => {0},
                                        _ => {1}, 
                                    }
                                  )
                                  .collect();

        //println!("Binary rep: {:?}", binary_rep);
        let mut num_rep = 0;
        for (i, br) in binary_rep.iter().enumerate() {
            if *br == 0 {
                let curr_pos = (binary_rep.len() - 1) - i;
                num_rep |= u32::pow(2, curr_pos as u32);
            }
        };

        //println!("num rep: {}", num_rep);

        let mut permuation_nums = Vec::new();

        for i in 0..u32::pow(2, self.data.len() as u32) {
            let val_masked = i & num_rep;
            if val_masked > 0 && !permuation_nums.contains(&val_masked){
                permuation_nums.push(val_masked);
            }
        }

        //println!("permuation_nums: {:?}", permuation_nums);

        for pn in permuation_nums {
            let mut arr = Vec::new();
            for i in (0..=binary_rep.len() - 1).rev() {
                //let curr_pos = (binary_rep.len() - 1) - i;
                if (pn & u32::pow(2, i as u32)) as u32 > 0 {
                    arr.push(SpringCondition::Functional);
                } else if binary_rep.get((binary_rep.len() - 1) - i).unwrap() == &1 {
                    arr.push(*self.data.get((binary_rep.len() - 1) - i).unwrap());
                } else {
                    arr.push(SpringCondition::Damaged);
                }
            }
            //println!("num: {}, arr: {:?}", pn, arr);
            arrangements.push(SpringArrangement { arrangement: arr });
        }

        return arrangements;
    }

    fn try_arrangement(&self, arrangement: SpringArrangement) -> Result<bool, &str> {
        let mut counts = Vec::with_capacity(self.info.len());
        //println!("arrangement: {:?}", arrangement);

        // State machine
        let mut is_counter_running = false;
        let mut i = 0;
        for sc in arrangement.arrangement {
            match sc {
                SpringCondition::Damaged => {
                    if is_counter_running {
                        is_counter_running = false;
                        counts.push(i);
                        i = 0;
                    }
                },
                SpringCondition::Functional => {
                    if !is_counter_running {
                        is_counter_running = true;
                    }

                    i += 1;
                },
                SpringCondition::Unknown => { return Err("Spring condtion should not be unknown here."); } 
            }
        }

        if is_counter_running {
            counts.push(i);
        }

        //println!("self.info: {:?}, counts: {:?}", self.info, counts);

        if self.info == counts {
            return Ok(true);
        }

        return Ok(false);
    }
}

pub fn sln(input_path: &str) -> u64 {
    let input = read_file_split_newlines(input_path);

    let mut records = Vec::new();

    for line in input {
        let spring_record = SpringRecord::new(&line);
        records.push(spring_record);
        //println!("{:?}", spring_record);
    }

    let mut result = 0;
    for (i, r) in records.iter().enumerate() {
        //println!("count: {}", r.get_num_of_arrangements());
        //println!("on record number: {} of {}", i + 1, records.len());
        let num_of_arrangements = r.get_num_of_arrangements();
        println!("{}", num_of_arrangements);
        result += num_of_arrangements;
    }

    return result as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_12/practice_input.txt"), 22);
    }

    /*
    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_12/input.txt"), 7407);
    }
    */
}