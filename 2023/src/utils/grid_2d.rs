
/// (Y, X)
const EIGHT_WAY_NEIGHBORS: [NeighborLocationData<i8>; 8] = [
    NeighborLocationData {point: Point {y: -1, x: -1}, direction: NeighborDirection::NorthWest},
    NeighborLocationData {point: Point {y: -1, x: 0},  direction: NeighborDirection::North},
    NeighborLocationData {point: Point {y: -1, x: 1},  direction: NeighborDirection::NorthEast},
    NeighborLocationData {point: Point {y: 0,  x: -1}, direction: NeighborDirection::West},
    NeighborLocationData {point: Point {y: 0,  x: 1},  direction: NeighborDirection::East},
    NeighborLocationData {point: Point {y: 1,  x: -1}, direction: NeighborDirection::SouthWest},
    NeighborLocationData {point: Point {y: 1,  x: 0},  direction: NeighborDirection::South},
    NeighborLocationData {point: Point {y: 1,  x: 1},  direction: NeighborDirection::SouthEast},
];

/// (Y, X)
const FOUR_WAY_NEIGHBORS: [NeighborLocationData<i8>; 4] = [
                            EIGHT_WAY_NEIGHBORS[1],
    EIGHT_WAY_NEIGHBORS[3],                         EIGHT_WAY_NEIGHBORS[4],
                            EIGHT_WAY_NEIGHBORS[6],
];

/// (Y, X)
const FOUR_WAY_NEIGHBORS_DIAGONAL: [NeighborLocationData<i8>; 4] = [
    EIGHT_WAY_NEIGHBORS[0],                        EIGHT_WAY_NEIGHBORS[2],
    EIGHT_WAY_NEIGHBORS[5],                        EIGHT_WAY_NEIGHBORS[7],
];

pub enum NeighborWay {
    FourWay,
    EightWay,
    FourWayDiagonal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeighborDirection {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NeighborLocationData<T> {
    point: Point<T>,
    direction: NeighborDirection,
}

#[derive(Debug, PartialEq)]
pub struct NeighborData<T> {
    loc: NeighborLocationData<usize>,
    data: Option<T>,
}

#[derive(Debug)]
pub struct Grid2D<T> {
    size_rows: usize,
    size_cols: usize,

    data: Vec<Vec<Option<T>>>,
}

impl<T: Copy> Grid2D<T> {
    pub fn new(num_rows: usize, num_cols: usize) -> Grid2D<T> {
        let mut grid: Grid2D<T> = Grid2D {
            size_rows: num_rows,
            size_cols: num_cols,
            data: vec![vec![None; num_cols]; num_rows]
        };

        return grid;
    }

    pub fn set(&mut self, loc: Point<usize>, data: &T) -> Result<(), &str>{
        if !self.bounds_check(Point { y: loc.y as i32, x: loc.x as i32 }) {
            return Err("Grid2D::set() Out of bounds.");
        }

        self.data[loc.y][loc.x] = Some(*data);

        return Ok(());
    }

    pub fn get(&self, p: Point<usize>) -> Result<Option<T>, &str> {
        if !self.bounds_check(Point { y: p.y as i32, x: p.x as i32 }) {
            return Err("Grid2D::set() Out of bounds.");
        }

        return Ok(self.data[p.y][p.x]);
    }

    /// Returns the size as (#rows, #cols)
    pub fn get_size(&self) -> (usize, usize) {
        return (self.size_rows, self.size_cols);
    }

    /// Will return the neighbor and position (in bounds)
    pub fn get_neighbors(&self, p: Point<usize>, way: NeighborWay) -> Vec<NeighborData<T>> {
        let mut result: Vec<NeighborData<T>> = Vec::new();

        // Points of interest
        let pois: &[NeighborLocationData<i8>];
        
        match way {
            NeighborWay::FourWay => {
                pois = &FOUR_WAY_NEIGHBORS;
            },
            NeighborWay::FourWayDiagonal => {
                pois = &FOUR_WAY_NEIGHBORS_DIAGONAL;
            },
            NeighborWay::EightWay => {
                pois = &EIGHT_WAY_NEIGHBORS;
            }
        };

        for location in pois {
            let p: Point<i32> = Point {
                y: p.y as i32 + location.point.y as i32,
                x: p.x as i32 + location.point.x as i32,
            };

            // Check for out of bounds
            if !self.bounds_check(p) {
                // Skip it
                continue;
            }

            // Convert to usize point
            let p_usize = Point {
                y: p.y as usize,
                x: p.x as usize,
            };

            // Its in bounds, add it to the vec
            let neighbor_data: NeighborData<T> = NeighborData {
                loc: NeighborLocationData {
                    point: p_usize,
                    direction: location.direction
                },
                data: self.get(p_usize).unwrap()
            };

            result.push(neighbor_data);
        }

        return result;
    }

    fn bounds_check(&self, p: Point<i32>) -> bool {
        // Less than Zero is handled by type
        return (p.y >= 0 && p.y < self.size_rows as i32) && (p.x >= 0 && p.x < self.size_cols as i32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_size() {
        let grid: Grid2D<u8> = Grid2D::new(42, 314);

        assert_eq!(grid.get_size(), (42, 314));
    }

    #[test]
    fn bounds_check() {
        let grid: Grid2D<u8> = Grid2D::new(10, 10);

        // Upper left in-bounds
        assert!(grid.bounds_check(Point { y: 0, x: 0 }));
        assert!(grid.bounds_check(Point { y: 1, x: 0 }));
        assert!(grid.bounds_check(Point { y: 0, x: 1 }));
        assert!(grid.bounds_check(Point { y: 1, x: 1 }));

        // Upper left out-of-bounds
        assert!(!grid.bounds_check(Point { y: -1, x:  0 }));
        assert!(!grid.bounds_check(Point { y:  0, x: -1 }));
        assert!(!grid.bounds_check(Point { y: -1, x: -1 }));

        // Lower left in-bounds
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 1, x: 0 }));
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 1, x: 1 }));
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 2, x: 0 }));
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 2, x: 1 }));

        // Lower left out-of-bounds
        assert!(!grid.bounds_check(Point { y: grid.size_rows as i32, x: 0 }));
        assert!(!grid.bounds_check(Point { y: grid.size_rows as i32, x: 1 }));
        assert!(!grid.bounds_check(Point { y: grid.size_rows as i32, x: -1 }));


        // Upper right in-bounds
        assert!(grid.bounds_check(Point { y: 0, x: grid.size_cols as i32 - 1 }));
        assert!(grid.bounds_check(Point { y: 1, x: grid.size_cols as i32 - 1 }));
        assert!(grid.bounds_check(Point { y: 0, x: grid.size_cols as i32 - 2 }));
        assert!(grid.bounds_check(Point { y: 1, x: grid.size_cols as i32 - 2 }));

        // Upper right out-of-bounds
        assert!(!grid.bounds_check(Point { y: -1, x: grid.size_cols as i32 - 1 }));
        assert!(!grid.bounds_check(Point { y:  0, x: grid.size_cols as i32 }));
        assert!(!grid.bounds_check(Point { y: -1, x: grid.size_cols as i32 }));

        // Lower right in-bounds
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 1, x: grid.size_cols as i32 - 1 }));
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 1, x: grid.size_cols as i32 - 2 }));
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 2, x: grid.size_cols as i32 - 1 }));
        assert!(grid.bounds_check(Point { y: grid.size_rows as i32 - 2, x: grid.size_cols as i32 - 2 }));

        // Lower right out-of-bounds
        assert!(!grid.bounds_check(Point { y: grid.size_rows as i32, x: grid.size_cols as i32 - 1 }));
        assert!(!grid.bounds_check(Point { y: grid.size_rows as i32, x: grid.size_cols as i32 }));
        assert!(!grid.bounds_check(Point { y: grid.size_rows as i32, x: grid.size_cols as i32 - 2 }));

    }

    #[test]
    fn four_way_neighbor() {
        let mut grid: Grid2D<u8> = Grid2D::new(10, 10);

        for y in 0..grid.data.len() {
            for x in 0..grid.data[y].len() {
                match grid.set(Point { y: y, x: x }, &((y * 10 + x) as u8)) {
                    Ok(_result) => {
                        // ignore
                    },
                    Err(_msg) => {
                        assert!(false);
                    }
                }
            }
        }

        assert_eq!(
            grid.get_neighbors(Point { y: 0, x: 0 }, NeighborWay::FourWay),
            vec![
                NeighborData {
                    loc: NeighborLocationData {
                        point: Point { y: 0, x: 1},
                        direction: NeighborDirection::East,
                    },
                    data: Some(1),
                },
                NeighborData {
                    loc: NeighborLocationData {
                        point: Point { y: 1, x: 0},
                        direction: NeighborDirection::South,
                    },
                    data: Some(10),
                }
            ]
        );

        assert_eq!(
            grid.get_neighbors(Point { y: 1, x: 1 }, NeighborWay::FourWay),
            vec![
                NeighborData {
                    loc: NeighborLocationData {
                        point: Point { y: 0, x: 1},
                        direction: NeighborDirection::North,
                    },
                    data: Some(1),
                },
                NeighborData {
                    loc: NeighborLocationData {
                        point: Point { y: 1, x: 0},
                        direction: NeighborDirection::West,
                    },
                    data: Some(10),
                },
                NeighborData {
                    loc: NeighborLocationData {
                        point: Point { y: 1, x: 2},
                        direction: NeighborDirection::East,
                    },
                    data: Some(12),
                },
                NeighborData {
                    loc: NeighborLocationData {
                        point: Point { y: 2, x: 1},
                        direction: NeighborDirection::South,
                    },
                    data: Some(21),
                }
            ]
        );
    }
}