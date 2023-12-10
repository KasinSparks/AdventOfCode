use std::fmt::{self, write};

use crate::utils::{io::read_file_split_newlines, grid_2d::{Grid2D, Point, NeighborWay, NeighborDirection}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}

impl PipeType {
    pub fn get_pipe_type(c: &char) -> Result<Self, ()> {
        let t = match c {
            '|' => { PipeType::Vertical },
            '-' => { PipeType::Horizontal },
            'L' => { PipeType::NorthEast },
            'J' => { PipeType::NorthWest },
            '7' => { PipeType::SouthWest },
            'F' => { PipeType::SouthEast },
            '.' => { PipeType::Ground },
            'S' => { PipeType::Start }, 
            _ => {
                println!("Error");
                return Err(());
            }
        };

        return Ok(t);
    }
}

impl fmt::Display for PipeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    } 
}

pub fn sln(input_path: &str) -> i32 {
    let input: Vec<String> = read_file_split_newlines(input_path);

    let mut grid = Grid2D::new(input.len(), input[0].len());

    let mut temp_loc = Point { x: 0, y: 0 };

    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let pt = PipeType::get_pipe_type(&c).unwrap();
            if pt == PipeType::Start {
                temp_loc = Point { x: col, y: row };
            }
            let _ = grid.set(Point { x: col, y: row }, &pt);
        }
    }

    //println!("{}", grid);

    let start_loc = temp_loc;
    let neighbors = grid.get_neighbors(start_loc, NeighborWay::FourWay);
    let mut start_headings= Vec::new();

    for n in neighbors {
        match n.loc.direction {
            NeighborDirection::North => {
                match n.data.unwrap() {
                    PipeType::Vertical | PipeType::SouthEast | PipeType::SouthWest => {
                        start_headings.push(NeighborDirection::North);
                    },
                    _ => { continue; }
                }
            },
            NeighborDirection::West => {
                match n.data.unwrap() {
                    PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast => {
                        start_headings.push(NeighborDirection::West);
                    },
                    _ => { continue; }
                }
            },
            NeighborDirection::East => {
                match n.data.unwrap() {
                    PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => {
                        start_headings.push(NeighborDirection::East);
                    },
                    _ => { continue; }
                }
            },
            NeighborDirection::South => {
                match n.data.unwrap() {
                    PipeType::Vertical | PipeType::NorthEast | PipeType::NorthWest => {
                        start_headings.push(NeighborDirection::South);
                    },
                    _ => { continue; }
                }
            },
            _ => { println!("err:"); return -1; }
        }
    }


    let mut trackers = (
        (start_loc, start_headings[0]),
        (start_loc, start_headings[1])
    );

    let mut i = 0;

    loop {
        println!("Trackers{:?}", trackers);
        trackers.0 = get_next_position(&grid, trackers.0.0, trackers.0.1).unwrap();
        trackers.1 = get_next_position(&grid, trackers.1.0, trackers.1.1).unwrap();

        if trackers.0.0 == start_loc || trackers.1.0 == start_loc {
            // trackers did not meet.
            println!("Trackers failed.");
            return -1;
        } else if trackers.0.0 == trackers.1.0 {
            println!("Trackers meet at: {:?}", trackers);
            i += 1;
            break;
        }
        
        i += 1;
    }

    println!("Point: {:?}, {:?}", trackers.0.0, trackers.1.0);

    return i;
}

fn get_next_position(grid: &Grid2D<PipeType>, point: Point<usize>, heading: NeighborDirection) -> Result<(Point<usize>, NeighborDirection), &str> {
    let neighbors = grid.get_neighbors(point, NeighborWay::FourWay);
    println!("neighbors: {:?}", neighbors);

    let mut next_heading = heading;

    let mut neighbor_addr = 0;
    for (i, n) in neighbors.iter().enumerate() {
        if heading == n.loc.direction {
            neighbor_addr = i;
        }
    }

    /*
    let neighbor_addr = match heading {
        NeighborDirection::North => {0},
        NeighborDirection::West  => {1},
        NeighborDirection::East  => {2},
        NeighborDirection::South => {3},
        _ => { return Err("Unable to match neighbor heading."); }
    };
    */

    let next_point = &neighbors[neighbor_addr];

    match neighbors[neighbor_addr].data.unwrap() {
        PipeType::Horizontal | PipeType::Vertical => {},
        PipeType::NorthWest => {
            match heading {
                NeighborDirection::South => {next_heading = NeighborDirection::West},
                NeighborDirection::East => {next_heading = NeighborDirection::North},
                _ => { return Err("1"); }
            }
        },
        PipeType::NorthEast => {
            match heading {
                NeighborDirection::South => {next_heading = NeighborDirection::East},
                NeighborDirection::West => {next_heading = NeighborDirection::North},
                _ => { return Err("2"); }
            }
        },
        PipeType::SouthWest => {
            match heading {
                NeighborDirection::North => {next_heading = NeighborDirection::West},
                NeighborDirection::East => {next_heading = NeighborDirection::South},
                _ => { return Err("3"); }
            }
        },
        PipeType::SouthEast => {
            match heading {
                NeighborDirection::North => {next_heading = NeighborDirection::East},
                NeighborDirection::West => {next_heading = NeighborDirection::South},
                _ => { return Err("4"); }
            }
        },
        _ => { return Err("Unknown Pipetype"); }
    }

    return Ok((next_point.loc.point, next_heading));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_result() {
        assert_eq!(sln("./src/days/day_10/practice_input.txt"), 4);
    }

    #[test]
    fn practice_result2() {
        assert_eq!(sln("./src/days/day_10/practice_input2.txt"), 8);
    }

    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_10/input.txt"), 6768);
    }
}