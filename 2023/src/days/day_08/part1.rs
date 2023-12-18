use crate::utils::{io::read_file_split_newlines, parsing::lexer::{Lexer, TokenType}};
use std::collections::HashMap;

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
}

pub fn sln(input_path: &str) -> u64 {
    let input: Vec<String> = read_file_split_newlines(input_path);

    // First line is Left right ins
    let lr_ins: Vec<char> = input[0].as_bytes().iter().map(|val| *val as char).collect();
    println!("ins: {:?}", lr_ins);

    // Skip blank line

    // get the node network
    let mut node_network: HashMap<Node, (Node, Node)> = HashMap::new();
    for line in &input[2..] {
        let lexer = Lexer::new(line.to_string());
        let lex_result = lexer.lex().unwrap();

        let nodes: Vec<Node> = lex_result.iter()
                                         .filter(|val| val.get_token_type() == TokenType::WORD)
                                         .map(|val| Node::new(val.get_data()))
                                         .collect();

        node_network.insert(nodes[0], (nodes[1], nodes[2]));
    }

    for node in &node_network {
        println!("Node: {:?}", node);
    }

    // Start at AAA and go until ZZZ
    let mut lr_ins_index = 0;
    let mut curr_node_key = Node::new("AAA");
    let mut i: u64 = 0;
    while curr_node_key != Node::new("ZZZ") {
        let lr = lr_ins[lr_ins_index];

        match lr {
            'L' => {curr_node_key = node_network.get(&curr_node_key).unwrap().0},
            'R' => {curr_node_key = node_network.get(&curr_node_key).unwrap().1},
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

    return i;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_08/practice_input.txt"), 2);
    }

    #[test]
    fn practice_result2() {
        assert_eq!(sln("./src/days/day_08/practice_input2.txt"), 6);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_08/input.txt"), 20513);
    }
}