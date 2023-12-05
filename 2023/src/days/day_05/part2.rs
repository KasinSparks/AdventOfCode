use std::collections::{HashSet, HashMap};

use crate::utils::{io::{read_file, read_file_split_newlines}, parsing::lexer::{Lexer, TokenType}, bytes_to_num::bytes_to_num};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum TransitionMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    UNK,
}

impl TransitionMapType {
    pub fn from_str(s: &str) -> Self {
        return match s {
            "seed-to-soil" => { Self::SeedToSoil },
            "soil-to-fertilizer" => { Self::SoilToFertilizer },
            "fertilizer-to-water" => { Self::FertilizerToWater },
            "water-to-light" => { Self::WaterToLight },
            "light-to-temperature" => { Self::LightToTemperature },
            "temperature-to-humidity" => { Self::TemperatureToHumidity },
            "humidity-to-location" => { Self::HumidityToLocation },
            _ => { Self::UNK }
        };
    }
}

#[derive(Clone, Copy, Debug)]
struct MapData {
    dest_range_start: usize,
    src_range_start: usize,
    range: usize,
}

impl MapData {
    pub fn new(drs: usize, srs: usize, r: usize) -> Self {
        return Self { dest_range_start: drs, src_range_start: srs, range: r };
    } 

    pub fn get_dest_val(&self, id: usize) -> Option<usize> {
        // Simple case
        if id >= self.src_range_start && id < self.src_range_start + self.range {
            let diff = id as i64 - self.src_range_start as i64 ;
            return Some((self.dest_range_start as i64 + diff) as usize);
        } else {
            // TODO: This may not be correct. The text says one thing but the 
            //       diagrams suggests another. 
            //return id;
            return None;
        }

    }
}

pub fn sln(input_path: &str) -> i32 {
    let input = read_file_split_newlines(input_path);

    //let mut seed_ids: HashSet<usize> = HashSet::new();
    let mut seed_ids: Vec<usize> = Vec::new();

    let mut maps: HashMap<TransitionMapType, Vec<MapData>> = HashMap::new();

    let mut map_type = TransitionMapType::UNK;

    let mut map_data = Vec::new();

    for (i, line) in input.iter().enumerate() {
        let lexer = Lexer::new(line.to_string());
        let tokens = lexer.lex().unwrap();

        //println!("i: {}, tokens: {:?}", i, tokens);

        if i == 0 {
            // First line is seed ids
            let it = tokens.iter().filter(|t| t.get_token_type() == TokenType::NUMBER);
            for number in it {
                //seed_ids.insert(bytes_to_num(number.get_data().as_bytes()).unwrap() as usize);
                seed_ids.push(bytes_to_num(number.get_data().as_bytes()).unwrap() as usize);
               
            }
        } else if tokens.len() == 0 {
            // Empty new line seperating the data
            //assert!(tokens[0].get_token_type() == TokenType::NEWLINE);

            if i != 1 {
                // Push the map data 
                maps.insert(map_type, map_data);
            }

            // Reset the placeholders
            map_type = TransitionMapType::UNK;
            map_data = Vec::new();
        } else {
            // Parse the map
            if tokens[0].get_token_type() == TokenType::WORD {
                // Save the map type
                let mut s: String = String::new();
                s.push_str(tokens[0].get_data());
                s.push_str(tokens[1].get_data());
                s.push_str(tokens[2].get_data());

                map_type = TransitionMapType::from_str(&s);
                assert!(map_type != TransitionMapType::UNK);
            } else {
                //println!("Token: {:?}", tokens);
                // Parse the ranges data
                let nums: Vec<usize> = tokens.iter()
                    .filter(|t| t.get_token_type() == TokenType::NUMBER)
                    .map(|tok| bytes_to_num(tok.get_data().as_bytes()).unwrap() as usize)
                    .collect();

                //println!("Nums: {:?}", nums);

                map_data.push(MapData::new(nums[0], nums[1], nums[2]));
            }
        }

    }

    // Add data if there is still data in the buffer
    if map_type != TransitionMapType::UNK {
        // Push the map data 
        maps.insert(map_type, map_data);

        // Reset the placeholders
        map_type = TransitionMapType::UNK;
        map_data = Vec::new();
    }

    println!("Seeds: {:#?}", seed_ids);
    println!("Maps: {:#?}", maps);

    let mut results = Vec::new();

    let mut actual_seed_ids: Vec<(usize, usize)> = Vec::new();

    let mut ii = 0;
    while ii < seed_ids.len() {
        actual_seed_ids.push((seed_ids[ii], seed_ids[ii + 1]));
        ii += 2;
    }

    println!("asis: {:#?}", actual_seed_ids);


    //let mut lowest_location_num = usize::MAX;
    let mut ss = 0;

    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = vec![];

    let mut tid = 0;
    for asi in actual_seed_ids {
        println!("starting asi with range: {}..{}", asi.0, asi.0 + asi.1);

        let datas = [
            maps.get(&TransitionMapType::SeedToSoil).unwrap().clone(),
            maps.get(&TransitionMapType::SoilToFertilizer).unwrap().clone(),
            maps.get(&TransitionMapType::FertilizerToWater).unwrap().clone(),
            maps.get(&TransitionMapType::WaterToLight).unwrap().clone(),
            maps.get(&TransitionMapType::LightToTemperature).unwrap().clone(),
            maps.get(&TransitionMapType::TemperatureToHumidity).unwrap().clone(),
            maps.get(&TransitionMapType::HumidityToLocation).unwrap().clone()
        ];

        //let datas2 = datas.clone();
        let tx1 = tx.clone();
        let handle = std::thread::spawn(move || {
            println!("Spawned thread[{}] for range {}..{}.", tid, asi.0, asi.0 + asi.1);
            let mut lowest_location_num = usize::MAX;
            for seed in (asi.0)..(asi.0 + asi.1) {
                let mut val: usize = seed;

                for d in &datas {
                    for t in d {
                        if let Some(v) = t.get_dest_val(val) {
                            val = v;
                            break;
                        }
                    }
                    //println!("Seed: {}, val: {}", seed, val);
                }

                //println!("Seed: {}, val: {}", seed, val);
                //results.push(val);
                if val < lowest_location_num {
                    lowest_location_num = val;
                    //ss = seed;
                }
            }

            println!("Thread[{}] done!", tid);
            tx1.send(lowest_location_num).unwrap();
        });

        tid += 1;

        handles.push(handle);

        //println!("Done with one asi");
    }

    let n = handles.len();

    for h in handles {
        h.join().unwrap();
        //println!("msg: {:#?}", rx.recv());
    }

    for i in 0..n {
        results.push(rx.recv().unwrap());
    }

    results.sort();

    //println!("seed number: {}", ss);
    //println!("result: {}", lowest_location_num);

    assert!(results[0] < i32::MAX as usize);
    return results[0] as i32;
    //assert!(lowest_location_num < i32::MAX as usize);
    //return lowest_location_num as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_05/practice_input.txt"), 46);
    }

    /*
    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_05/input.txt"), );
    }
    */
}