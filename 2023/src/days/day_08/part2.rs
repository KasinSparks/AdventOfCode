use crate::utils::{io::read_file_split_newlines, parsing::lexer::{Lexer, TokenType}};
use std::{collections::HashMap};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Node {
    ins: [char; 3] 
}

impl Node {
    pub fn new(data: &str) -> Self {
        let mut result = Self {ins: ['\0', '\0', '\0']};
        let d = &data[0..3].as_bytes();

        for i in 0..3 {
            result.ins[i] = d[i] as char;
        }

        return result;
    }

    pub fn from(data: &[u8]) -> Self {
        return Self { ins: [data[0] as char, data[1] as char, data[2] as char] };
    }
}

pub fn sln(input_path: &str) -> u64 {
    let input: Vec<String> = read_file_split_newlines(input_path);

    // First line is Left right ins
    let lr_ins: Vec<char> = input[0].as_bytes().iter().map(|val| *val as char).collect();
    //println!("ins: {:?}", lr_ins);

    // Skip blank line

    // get the node network
    let mut node_network: HashMap<Node, (Node, Node)> = HashMap::new();
    for line in &input[2..] {
        let line_str = line.as_bytes();

        let nodes: Vec<Node> = vec![
            Node::from(&line_str[0..3]),
            Node::from(&line_str[7..10]),
            Node::from(&line_str[12..15])
        ];

        node_network.insert(nodes[0], (nodes[1], nodes[2]));
    }

    /*
    for node in &node_network {
        println!("Node: {:?}", node);
    }
    */

    
    // Start at XXA and go until XXZ
    let mut lr_ins_index = 0;
    //let mut curr_node_keys: Vec<&Node> = node_network.keys().filter(|&key| key.ins[2] == 'A').collect();
    let mut i: u64 = 0;



    // Start at XXA and go until XXZ
    let temp_node_keys: Vec<&Node> = node_network.keys().filter(|&key| key.ins[2] == 'A').collect();

    let mut curr_node_keys: Vec<(&Node, u64)> = temp_node_keys.iter().map(|&k| (k, 0)).collect();

    /*
    for node in &curr_node_keys {
        println!("curr_nodes: {:?}", node);
    }
    */

    for key in &mut curr_node_keys {
        let mut i: u64 = 0;
        let mut lr_ins_index = 0;
        let mut curr_node = *key.0;
        while curr_node.ins[2] != 'Z' {
            let lr = lr_ins[lr_ins_index];

            match lr {
                'L' => {curr_node = node_network.get(&curr_node).unwrap().0},
                'R' => {curr_node = node_network.get(&curr_node).unwrap().1},
                _ => { panic!("unknown lr ins."); }
            }

            if lr_ins_index >= lr_ins.len() - 1 {
                lr_ins_index = 0;
            } else {
                lr_ins_index += 1;
            }


            //println!("curr ins: {}, curr_node_key: {:?}", lr, curr_node_key);
            i += 1;
        }

        key.1 = i;
    }

    //println!("r: {:?}", curr_node_keys);

    let mut result: usize = 1;
    for key in &curr_node_keys {
        //result *= key.1;
        result = lcm(result, key.1 as usize);
    }

    return result as u64;
}

// BEGIN: CODE FROM https://www.hackertouch.com/least-common-multiple-in-rust.html
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					
// END: CODE FROM https://www.hackertouch.com/least-common-multiple-in-rust.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_08/practice_input3.txt"), 6);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_08/input.txt"), 15995167053923);
    }
}