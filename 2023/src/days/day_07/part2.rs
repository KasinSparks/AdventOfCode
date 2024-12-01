use std::{collections::{HashMap, BTreeMap}, cmp::Ordering};

use crate::utils::{io::read_file_split_newlines, bytes_to_num::bytes_to_num};

// TOO HIGH 253868339
// TOO HIGH 253867376
// TOO LOW  248281607

#[derive(Debug, PartialOrd, Eq)]
struct Card {
    value: u8
}

impl Card {
    pub fn new(c: char) -> Self {
        let val = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '2'..='9' => c as u8 - '0' as u8,
            'J' => 0,
            _ => 0
        };

        return Self {value: val};
    } 

    pub fn get_value(&self) -> u8 {
        return self.value;
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.get_value().cmp(&other.get_value());
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.get_value() == other.get_value(); 
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    pub fn get_ranking(&self) -> u8 {
        return match self {
            Self::FiveOfAKind => 6,
            Self::FourOfAKind => 5,
            Self::FullHouse => 4,
            Self::ThreeOfAKind => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.get_ranking().cmp(&other.get_ranking());
    }
}


#[derive(Debug, Eq, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType
}

impl Hand {
    pub fn new(raw_data: &str) -> Result<Self, &str> {
        if raw_data.len() != 5 {
            return Err("Too few cards.");
        }

        let mut cards = Vec::new();

        for c in raw_data.chars() {
            cards.push(Card::new(c));
        }

        // Determine hand type
        let mut labels: HashMap<u8, u8>= HashMap::new();
        for c in &cards {
            if let Some(v) = labels.get_mut(&c.get_value()) {
                *v += 1;
            } else {
                labels.insert(c.value, 1);
            }
        }

        if let Some(joker) = labels.get(&0) {
            let mut temp_map = HashMap::new();

            //let max_key = labels.keys().max().unwrap();
            let mut largest_set: (u8, u8) = (0, 0);
            for l in labels.keys() {
                if l != &0 {
                    let temp_val = labels.get(l).unwrap();
                    if temp_val > &largest_set.1 {
                        largest_set = (*l, *temp_val);
                    } else if temp_val == &largest_set.1 {
                        if l > &largest_set.0 {
                            largest_set = (*l, *temp_val);
                        }
                    }
                }
            }

            if largest_set.0 == 0 {
                temp_map.insert(0, 5);
            } else {
                temp_map.insert(largest_set.0, labels.get(&largest_set.0).unwrap() + *joker);

                for key in labels.keys() {
                    if *key != 0 && *key != largest_set.0 {
                        temp_map.insert(*key, *(labels.get(key).unwrap()));
                    }
                }
            }

            labels = temp_map;
        }


        let mut temp_vec = Vec::new();
        for l in labels.values() {
            temp_vec.push(l);
        }

        temp_vec.sort();

        let hand_type = match temp_vec.as_slice() {
            [5]  => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => HandType::HighCard
        };

        return Ok(Self { cards: cards, hand_type: hand_type });
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        //return self.hand_type.cmp(&other.hand_type);
        if self.hand_type == other.hand_type {
            // Compare the cards one by one
            for (i, c) in self.cards.iter().enumerate() {
                if c < &other.cards[i] {
                    return Ordering::Less;
                } else if c > &other.cards[i] {
                    return Ordering::Greater;
                }
            }

            return Ordering::Equal;
        } else {
            return self.hand_type.cmp(&other.hand_type);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.hand_type == other.hand_type;
    }
}

pub fn sln(input_path: &str) -> u64 {
    let input: Vec<String> = read_file_split_newlines(input_path);

    let mut players = Vec::new();

    for line in &input {
        let vals: Vec<&str> = line.split(' ').collect();

        players.push((Hand::new(vals[0]).unwrap(), bytes_to_num(vals[1].as_bytes()).unwrap()));
    }

    // bucket
    players.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result: u64 = 0;

    for (i, p) in (&players).iter().enumerate() {
        //println!("{}: {:?}", i, p);
        result += ((i + 1) * p.1) as u64;
    }

    return result; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_07/practice_input.txt"), 5905);
    }

    #[test]
    fn practice_result2() {
        assert_eq!(sln("./src/days/day_07/practice_input2.txt"), 3667);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_07/input.txt"), 253718286);
    }
}