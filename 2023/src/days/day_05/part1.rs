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

/*
struct TransitionMap {
    map_type: TransitionMapType,
    data: Vec<MapData>,
}

impl TransitionMap {
    pub fn new() -> Self {
        return Self { map_type: TransitionMapType::UNK, data: Vec::new() };
    } 
}
*/

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

    let mut seed_ids: HashSet<usize> = HashSet::new();

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
                seed_ids.insert(bytes_to_num(number.get_data().as_bytes()).unwrap() as usize);
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

    for seed in seed_ids {
        let mut val: usize = seed;

        let mut datas = Vec::new();

        datas.push(maps.get(&TransitionMapType::SeedToSoil).unwrap());
        datas.push(maps.get(&TransitionMapType::SoilToFertilizer).unwrap());
        datas.push(maps.get(&TransitionMapType::FertilizerToWater).unwrap());
        datas.push(maps.get(&TransitionMapType::WaterToLight).unwrap());
        datas.push(maps.get(&TransitionMapType::LightToTemperature).unwrap());
        datas.push(maps.get(&TransitionMapType::TemperatureToHumidity).unwrap());
        datas.push(maps.get(&TransitionMapType::HumidityToLocation).unwrap());

        for d in datas {
            for t in d {
                if let Some(v) = t.get_dest_val(val) {
                    val = v;
                    break;
                }
            }
            //println!("Seed: {}, val: {}", seed, val);
        }

        println!("Seed: {}, val: {}", seed, val);
        results.push(val);
    }

    results.sort();

    println!("result: {}", results[0]);

    assert!(results[0] < i32::MAX as usize);
    return results[0] as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_05/practice_input.txt"), 35);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_05/input.txt"), 3374647);
    }
}