use std::{collections::{HashSet, BinaryHeap, BTreeSet}, cmp::Reverse};

use crate::utils::{io::read_file_split_newlines, grid_2d::{Point, Grid2D, NeighborWay}};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct PointWithWeight {
    weight: i32,
    point: Point<usize>
}

impl Ord for PointWithWeight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    } 
}

pub fn sln(input_path: &str) -> i32 {
    let input = read_file_split_newlines(input_path);

    let expanded = expander(&input);

    let mut points: Vec<Point<usize>> = Vec::new();

    for (row, line) in expanded.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                points.push(Point { x: col, y: row });
            }
        }
    }

    let mut grid = Grid2D::new(expanded.len(), expanded.get(0).unwrap().len());
    for p in &points {
        grid.set(*p, &'#');
    }

    //let mut distances: Vec<(Point<i32>, Point<i32>, i32)> = Vec::new();
    let mut total_steps = 0;

    for (i, ref_p) in points.iter().enumerate() {
        for target_p in &points[i+1..points.len()] {
            let mut nodes_checked: HashSet<Point<usize>> = HashSet::new();
            //let mut node_queue = Vec::new();
            let mut node_queue = BinaryHeap::new();
            let mut curr_point: Point<usize> = *ref_p;

            let mut steps = 0;

            //println!();
            //println!("curr_point: {:?}, ", curr_point);

            while curr_point != *target_p {
                if !nodes_checked.contains(&curr_point) {
                    let neighbors = grid.get_neighbors(curr_point, NeighborWay::FourWay);
                    for n in neighbors {
                        if !nodes_checked.contains(&n.loc.point) {
                            let dx = n.loc.point.x as i32 - target_p.x as i32;
                            let dy = n.loc.point.y as i32 - target_p.y as i32;
                            let weight = (dx * dx) + (dy * dy);
                            let pww = (weight, n.loc.point) ;
                            node_queue.push(Reverse(pww));
                        }
                    }

                    if node_queue.is_empty() && curr_point != *target_p {
                        println!("unable to find node: {:?}", target_p);
                        break;
                    }

                    nodes_checked.insert(curr_point);
                }
                //println!("heap before pop:({}) {:?}", node_queue.len(), node_queue);
                curr_point = node_queue.pop().unwrap().0.1;
                steps += 1;
                //println!("heap after pop:({}) {:?}", node_queue.len(), node_queue);
                //println!()
            }

            //println!("found target: {:?} in {} steps", target_p, steps);
            total_steps += steps;
        }
    }

    //println!("Points: {:#?}", points);

    /*
    //let mut distances: Vec<(Point<i32>, Point<i32>, f64)> = Vec::new();
    let mut distances: Vec<(Point<i32>, Point<i32>, i32)> = Vec::new();

    for (i, ref_p) in points.iter().enumerate() {
        for other_p in &points[i+1..points.len()] {

            //distances.push((*ref_p, *other_p, f64::sqrt(((dx * dx) + (dy * dy)) as f64)));
            distances.push((*ref_p, *other_p, i32::abs(dx + dy)));
        }
    }


    let mut result: i32 = 0;
    for d in &distances {
        println!("{:?}", d);
        result += d.2;
    }

    println!("result: {}", result);
    println!("# of pairs: {}", distances.len());
    */

    return total_steps;
}

fn expander(input: &Vec<String>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();

    // Horizontal
    for line in input {
        let mut is_empty = true;
        for c in line.chars() {
            if c != '.' {
                is_empty = false;
                break;
            }
        } 

        new_vec.push(line.to_string());

        if is_empty {
            let mut s = String::new();
            for i in 0..line.len() {
                s.push('.');
            }
            new_vec.push(s);
        }
    }

    // Vertical
    let mut pos = Vec::new();
    for i in 0..new_vec.get(0).unwrap().len() {
        let mut is_empty = true;
        //let mut vertical_line = String::new();

        for j in 0..new_vec.len() {
            let c = new_vec.get(j).unwrap().as_bytes()[i] as char;
            //vertical_line.push(c);
            if c != '.' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            pos.push(i);
        }
    }

    pos.reverse();

    for p in pos {
        for j in 0..new_vec.len() {
            new_vec.get_mut(j).unwrap().insert(p, '.');
        }
    }


    return new_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expander_test() {
        let expected = vec![
            "....#........",
            ".........#...",
            "#............",
            ".............",
            ".............",
            "........#....",
            ".#...........",
            "............#",
            ".............",
            ".............",
            ".........#...",
            "#....#......."
        ];

        assert_eq!(expected, expander(&read_file_split_newlines("./src/days/day_11/practice_input.txt")));
    }

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_11/practice_input.txt"), 374);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_11/input.txt"), 9639160);
    }
}