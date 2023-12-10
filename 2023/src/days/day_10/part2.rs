use std::{fmt::{self, write}, collections::HashSet};

use crate::utils::{io::{read_file_split_newlines, StringColor}, grid_2d::{Grid2D, Point, NeighborWay, NeighborDirection}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
    Marked
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
            'X' => { PipeType::Marked}, 
            _ => {
                println!("Error");
                return Err(());
            }
        };

        return Ok(t);
    }

    pub fn get_char(pt: Self) -> Result<char, ()> {
        let t = match pt {
            PipeType::Vertical   => '|',
            PipeType::Horizontal => '-',
            PipeType::NorthEast  => 'L',
            PipeType::NorthWest  => 'J',
            PipeType::SouthWest  => '7',
            PipeType::SouthEast  => 'F',
            PipeType::Ground     => '.',
            PipeType::Start      => 'S',
            PipeType::Marked     => 'X',
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

    let mut paths = (Vec::new(), Vec::new());
    paths.0.push(trackers.0);
    paths.1.push(trackers.1);

    let mut i = 0;

    loop {
        //println!("Trackers{:?}", trackers);
        trackers.0 = get_next_position(&grid, trackers.0.0, trackers.0.1).unwrap();
        trackers.1 = get_next_position(&grid, trackers.1.0, trackers.1.1).unwrap();

        paths.0.push(trackers.0);
        paths.1.push(trackers.1);

        if trackers.0.0 == start_loc || trackers.1.0 == start_loc {
            // trackers did not meet.
            println!("Trackers failed.");
            return -1;
        } else if trackers.0.0 == trackers.1.0 || trackers.0.0 == paths.1[paths.1.len() - 1].0 {
            println!("Trackers meet at: {:?}", trackers);
            i += 1;
            break;
        }
        
        i += 1;
    }

    //println!("path: {:?}", paths);
    let mut temp_grid: Grid2D<char> = Grid2D::new(grid.get_size().0, grid.get_size().1);
    for row in 0..temp_grid.get_size().0 {
        for col in 0..temp_grid.get_size().1 {
            temp_grid.set(Point { x: col, y: row }, &'.');
        }
    }

    for p in paths.0.iter() {
        temp_grid.set(p.0, &PipeType::get_char(grid.get(p.0).unwrap().unwrap()).unwrap());
    }

    for p in paths.1.iter() {
        temp_grid.set(p.0, &PipeType::get_char(grid.get(p.0).unwrap().unwrap()).unwrap());
    }

    //println!("Point: {:?}, {:?}", trackers.0.0, trackers.1.0);

    //println!("Grid: {}", temp_grid);
    //temp_grid.print_raw();

    let mut colorize_points = Vec::new();

    for row in 0..temp_grid.get_size().0 {
        for col in 0..temp_grid.get_size().1 {
            if temp_grid.get(Point { x: col, y: row }).unwrap().unwrap() == '.' {
                let neighbors = temp_grid.get_neighbors(Point { x: col, y: row }, NeighborWay::EightWay);
                for n in neighbors {
                    if n.data.unwrap() != '.' {
                        colorize_points.push(n.loc.point);
                        //break;
                    }
                }
            }
        }
    }

    for row in 0..temp_grid.get_size().0 {
        for col in 0..temp_grid.get_size().1 {
            let p = Point {x: col, y: row};
            /*
            let mut s = String::from(
                match temp_grid.get(p).unwrap().unwrap() {
                    '-' => "─",
                    'L' => "└",
                    'J' => "┘",
                    '|' => "│",
                    'F' => "┌",
                    '7' => "┐",
                    _ => "."
                }
            );
            */
            let mut s = String::from(temp_grid.get(p).unwrap().unwrap());
            if colorize_points.contains(&p) {
                s = StringColor::colorize_string(&s, StringColor::BrightGreen);
            }

            print!("{}", s);
        }
        println!();
    }

    /*
            ^
            |
       <--- . --->
            |
            V
    */

    /*
    for row in 0..temp_grid.get_size().0 {
        for col in 0..temp_grid.get_size().1 {
            if temp_grid.get(Point { x: col, y: row }).unwrap().unwrap() == '.' {

            }
        }
    }
    */

    let mut complete_path = Vec::new();
    complete_path.append(&mut paths.0[1..].to_vec());
    paths.1.reverse();
    complete_path.append(&mut paths.1[1..].to_vec());

    let actual_complete_path: Vec<Point<usize>> = complete_path.iter().map(|&val| val.0).collect();

    //let mut test_grid: Grid2D<char> = Grid2D::new(grid.get_size().0, grid.get_size().1);
    let mut left_values: Vec<Point<usize>> = Vec::new();
    let mut right_values: Vec<Point<usize>> = Vec::new();

    for row in 0..grid.get_size().0 {
        for col in 0..grid.get_size().1 {
            let p = Point {x: col, y: row};
            //temp_grid.set(p, &' ');
            if actual_complete_path.contains(&p) {
                let mut s = String::from(
                    match PipeType::get_char(grid.get(p).unwrap().unwrap()).unwrap() {
                        '-' => "─",
                        'L' => "└",
                        'J' => "┘",
                        '|' => "│",
                        'F' => "┌",
                        '7' => "┐",
                        'X' => "X",
                        _ => "."
                    }
                );
                print!("{}", s);
            } else {
                print!("{}", '.');
            }
        }
        println!();
    }

    let mut path_grid: Grid2D<PipeType> = Grid2D::new(grid.get_size().0, grid.get_size().1);

    for row in 0..grid.get_size().0 {
        for col in 0..grid.get_size().1 {
            let p = Point {x: col, y: row};
            //temp_grid.set(p, &' ');
            if actual_complete_path.contains(&p) {
                path_grid.set(p, &grid.get(p).unwrap().unwrap());
            } else {
                path_grid.set(p, &PipeType::Ground);
            }
        }
    }

    for p in &paths.0[1..] {
        let loc = (match p.1 {
            NeighborDirection::North => Point{ x: p.0.x - 1, y: p.0.y},
            NeighborDirection::South => Point{ x: p.0.x + 1, y: p.0.y},
            NeighborDirection::East => Point{ x: p.0.x, y: p.0.y - 1},
            NeighborDirection::West => Point{ x: p.0.x, y: p.0.y + 1},
            _ => {println!("Err: 5"); return 0;}
        });

        ///match grid.get(loc).unwrap().unwrap() {
        match path_grid.get(loc).unwrap().unwrap() {
            PipeType::Ground => {right_values.push(loc);},
            _ => {}
        };
    }

    for p in &paths.1[1..] {
        let loc = (match p.1 {
            NeighborDirection::North => Point{ x: p.0.x + 1, y: p.0.y + 0},
            NeighborDirection::South => Point{ x: p.0.x - 1, y: p.0.y + 0},
            NeighborDirection::East => Point{ x: p.0.x, y: p.0.y + 1},
            NeighborDirection::West => Point{ x: p.0.x, y: p.0.y - 1},
            _ => {println!("Err: 5"); return 0;}
        });

        ///match grid.get(loc).unwrap().unwrap() {
        match path_grid.get(loc).unwrap().unwrap() {
            PipeType::Ground => {left_values.push(loc);},
            _ => {}
        };
    }

    left_values.append(&mut right_values);



    for row in 0..grid.get_size().0 {
        for col in 0..grid.get_size().1 {
            let p = Point {x: col, y: row};
            //temp_grid.set(p, &' ');
            if left_values.contains(&p) {
                print!("X");
            } else if actual_complete_path.contains(&p) {
                let mut s = String::from(
                    match PipeType::get_char(grid.get(p).unwrap().unwrap()).unwrap() {
                        '-' => "─",
                        'L' => "└",
                        'J' => "┘",
                        '|' => "│",
                        'F' => "┌",
                        '7' => "┐",
                        'S' => "S",
                        'X' => "X",
                        _ => "."
                    }
                );
                print!("{}", s);

                //print!("{}", PipeType::get_char(grid.get(p).unwrap().unwrap()).unwrap());
            } else {
                print!("{}", ' ');
            }
        }
        println!();
    }

    let mut is_outside = false;
    // Determine if we got the outside or inside
    for p in &left_values {
        // left
        if p.x == 0 {
            is_outside = true;
            println!("line: 331");
            break;
        }

        let mut tp = Point{x: p.x - 1, y: p.y};
        while temp_grid.get(tp).unwrap().unwrap() == '.' && !is_outside {
            if tp.x == 0 {
                is_outside = true;
            println!("line: 339");
                break;
            } else {
                tp = Point{x: tp.x - 1, y: tp.y};
            }
        }

       
        // right
        if p.x == temp_grid.get_size().1 - 1{
            is_outside = true;
            println!("line: 350");
            break;
        }
        tp = Point{x: p.x + 1, y: p.y};
        while temp_grid.get(tp).unwrap().unwrap() == '.' && !is_outside {
            if tp.x == temp_grid.get_size().1 - 1{
                is_outside = true;
            println!("line: 357");
                break;
            } else {
                tp = Point{x: tp.x + 1, y: tp.y};
            }
        }

        // down
        if p.y == temp_grid.get_size().0 - 1{
            is_outside = true;
            println!("line: 367");
            break;
        }
        tp = Point{x: p.x, y: p.y + 1};
        while temp_grid.get(tp).unwrap().unwrap() == '.' && !is_outside {
            if tp.y == temp_grid.get_size().0 - 1 {
                is_outside = true;
            println!("line: 374, Point: {:?}, OgPoint: {:?}", tp, Point{x: p.x, y: p.y + 1});
                break;
            } else {
                tp = Point{x: tp.x, y: tp.y + 1};
            }
        }

        // up 
        if p.y == 0 {
            is_outside = true;
            println!("line: 384");
            break;
        }
        tp = Point{x: p.x, y: p.y - 1};
        while temp_grid.get(tp).unwrap().unwrap() == '.' && !is_outside {
            if tp.y == 0 {
                is_outside = true;
            println!("line: 391");
                break;
            } else {
                tp = Point{x: tp.x, y: tp.y - 1};
            }
        }
    }

    println!("is outside: {}", is_outside);

    let mut additionals = HashSet::new();

    let mut to_check: Vec<Point<usize>> = Vec::new();

    for p in &left_values {
        to_check.push(*p);
    }

    while !to_check.is_empty() {
        let tp = to_check.pop().unwrap();

        if temp_grid.get(tp).unwrap().unwrap() == '.' {
            if !additionals.contains(&tp) {
                additionals.insert(tp);
                for n in temp_grid.get_neighbors(tp, NeighborWay::EightWay) {
                    if !additionals.contains(&n.loc.point) {
                        to_check.push(n.loc.point);
                    }
                }
            }

        }

    }

    for lv in &left_values {
        additionals.insert(*lv);
    }


    //left_values.append(&mut additionals);

    // fill
    /*
    for p in &left_values {
        for i in (0..p.x).rev() {
            let temp_val = temp_grid.get(Point { x: i, y: p.y }).unwrap().unwrap();
            if temp_val == '.' || temp_val == 'X' {
                //temp_grid.set(Point { x: i, y: p.y }, &'X');
                additionals.push(Point { x: i, y: p.y });
            } else {
                break;
            }
        }
        
        for i in (p.x + 1)..temp_grid.get_size().1 {
            let temp_val = temp_grid.get(Point { x: i, y: p.y }).unwrap().unwrap();
            if temp_val == '.' || temp_val == 'X' {
                additionals.push(Point { x: i, y: p.y });
            } else {
                break;
            }
        }
    }

    left_values.append(&mut additionals);

    for p in &left_values {
        for i in (0..p.y).rev() {
            let temp_val = temp_grid.get(Point { x: p.x, y: i }).unwrap().unwrap();
            if temp_val == '.' || temp_val == 'X' {
                //temp_grid.set(Point { x: i, y: p.y }, &'X');
                additionals.push(Point { x: p.x, y: i });
            } else {
                break;
            }
        }
        
        for i in (p.y + 1)..temp_grid.get_size().0 {
            let temp_val = temp_grid.get(Point { x: p.x, y: i }).unwrap().unwrap();
            if temp_val == '.' || temp_val == 'X' {
                additionals.push(Point { x: p.x, y: i });
            } else {
                break;
            }
        }
    }

    left_values.append(&mut additionals);
    */
    let mut final_grid = Grid2D::new(grid.get_size().0, grid.get_size().1);

    for row in 0..grid.get_size().0 {
        for col in 0..grid.get_size().1 {
            let p = Point {x: col, y: row};
            //temp_grid.set(p, &' ');
            //if left_values.contains(&p) {
            if additionals.contains(&p) {
                let t = final_grid.set(p, &'X');
                if t.is_err() {
                    println!("ERRRRRRR");
                }
            } else if actual_complete_path.contains(&p) {
                let g_val = PipeType::get_char(grid.get(p).unwrap().unwrap()).unwrap();
                let mut s = String::from(
                    match g_val {
                        '-' => "─",
                        'L' => "└",
                        'J' => "┘",
                        '|' => "│",
                        'F' => "┌",
                        '7' => "┐",
                        'S' => "S",
                        'X' => "X",
                        _ => "."
                    }
                );
                //print!("{}", s);
                let t = final_grid.set(p, &g_val);
                if t.is_err() {
                    println!("ERRRRRRR");
                }

                //print!("{}", PipeType::get_char(grid.get(p).unwrap().unwrap()).unwrap());
            } else {
                //print!("{}", '.');
                let t = final_grid.set(p, &'.');
                if t.is_err() {
                    println!("ERRRRRRR");
                }
            }
        }
    }

    /* 
    println!("count: {}", left_values.len());

    let mut hm = HashSet::new();

    for val in left_values {
        hm.insert(val);
    }
    println!("count: {}", hm.len());
    

    return hm.len() as i32;
    */
    for row in 0..final_grid.get_size().0 {
        for col in 0..final_grid.get_size().1 {
            let p = Point {x: col, y: row};
            //temp_grid.set(p, &' ');
            //if left_values.contains(&p) {
            let s = final_grid.get(p).unwrap().unwrap();
            let o = match s {
                '-' => "─",
                'L' => "└",
                'J' => "┘",
                '|' => "│",
                'F' => "┌",
                '7' => "┐",
                'S' => "S",
                'X' => "X",
                _ => "*"
            };
            print!("{}", o);
        }
        println!();
    }

    if is_outside {
        let mut num_dots = 0;
        for row in 0..final_grid.get_size().0 {
            for col in 0..final_grid.get_size().1 {
                let p = Point {x: col, y: row};
                let v = final_grid.get(p).unwrap().unwrap();
                if v == '.' {
                    num_dots += 1;
                }
            }
        }

        return num_dots;
    }

    return additionals.len() as i32;
}

fn expander(grid: &Grid2D<PipeType>) -> Result<Grid2D<PipeType>, &str> {
    let old_grid_size = grid.get_size();
    let expaned_grid: Grid2D<PipeType> = Grid2D::new(old_grid_size.0 + (old_grid_size.0 - 1), old_grid_size.1 + (old_grid_size.1 - 1));

    for col in 0..grid.get_size().1 {
        let mut j = 0;
        while j < grid.get_size().0 - 1 {

            j += 1;
        }

    }

    for row in 0..grid.get_size().0  - 1{
    }

    return Ok(expaned_grid);
}

fn get_next_position(grid: &Grid2D<PipeType>, point: Point<usize>, heading: NeighborDirection) -> Result<(Point<usize>, NeighborDirection), &str> {
    let neighbors = grid.get_neighbors(point, NeighborWay::FourWay);
    //println!("neighbors: {:?}", neighbors);

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
        assert_eq!(sln("./src/days/day_10/p2_practice_input.txt"), 4);
    }

    #[test]
    fn practice_result2() {
        assert_eq!(sln("./src/days/day_10/p2_practice_input2.txt"), 4);
    }

    #[test]
    fn practice_result3() {
        assert_eq!(sln("./src/days/day_10/p2_practice_input3.txt"), 8);
    }

    #[test]
    fn practice_result4() {
        assert_eq!(sln("./src/days/day_10/p2_practice_input4.txt"), 10);
    }

    /*
    // 247 too low
    // 350 too low
    // 364 too high
    #[test]
    fn final_result() {
        assert_eq!(sln("./src/days/day_10/input.txt"), 0);
    }
    */
}