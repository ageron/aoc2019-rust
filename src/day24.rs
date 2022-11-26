use std::collections::HashSet;

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(grid: &[Vec<bool>]) -> Self {
        Self {
            grid: grid.to_vec(),
        }
    }

    fn step(&self) -> Self {
        let mut neighbors: Vec<Vec<usize>> = (0..5).map(|_| vec![0; 5]).collect();
        for y in 0..5 {
            for x in 0..5 {
                if !self.grid[y][x] {
                    continue;
                }
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || ny < 0 || nx > 4 || ny > 4 {
                        continue;
                    }
                    neighbors[ny as usize][nx as usize] += 1;
                }
            }
        }
        let new_grid_data = (0..5)
            .map(|y| {
                (0..5)
                    .map(|x| {
                        let count = neighbors[y][x];
                        count == 1 || (!self.grid[y][x] && count == 2)
                    })
                    .collect()
            })
            .collect();
        Self {
            grid: new_grid_data,
        }
    }

    fn biodiversity_rating(&self) -> usize {
        let mut result = 0;
        let mut power = 1;
        for y in 0..5 {
            for x in 0..5 {
                if self.grid[y][x] {
                    result |= power;
                }
                power <<= 1;
            }
        }
        result
    }

    fn display(&self) {
        for y in 0..5 {
            for x in 0..5 {
                print!("{}", if self.grid[y][x] { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }

    fn simulate_until_repetition(&self) -> Self {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut grid = self.clone();
        loop {
            let rating = grid.biodiversity_rating();
            if visited.contains(&rating) {
                return grid;
            }
            visited.insert(rating);
            grid = grid.step();
        }
    }
}

#[derive(Clone)]
struct RecursiveGrid {
    grids: Vec<Vec<Vec<bool>>>,
    lowest_level: isize,
}

impl RecursiveGrid {
    fn new(grid: &[Vec<bool>]) -> Self {
        Self {
            grids: vec![grid.to_vec()],
            lowest_level: 0,
        }
    }

    fn step(&self) -> Self {
        let mut neighbors: Vec<Vec<Vec<usize>>> = (0..self.grids.len() + 2)
            .map(|_| (0..5).map(|_| vec![0; 5]).collect())
            .collect();
        let mut min_z = 1;
        let mut max_z = self.grids.len();
        for z in 1..self.grids.len() + 1 {
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    if !self.grids[z - 1][y][x] {
                        continue;
                    }
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let mut nz = z;
                        let mut nx = x as isize + dx;
                        let mut ny = y as isize + dy;
                        if nx < 0 {
                            nz -= 1;
                            nx = 1;
                            ny = 2;
                        } else if nx > 4 {
                            nz -= 1;
                            nx = 3;
                            ny = 2;
                        } else if ny < 0 {
                            nz -= 1;
                            nx = 2;
                            ny = 1;
                        } else if ny > 4 {
                            nz -= 1;
                            nx = 2;
                            ny = 3;
                        }
                        if nx == 2 && ny == 2 {
                            nz += 1;
                            if nz > max_z {
                                max_z = nz;
                            }
                            if dx == 0 {
                                ny = if dy == -1 { 4 } else { 0 };
                                for nx in 0..5 {
                                    neighbors[nz][ny as usize][nx as usize] += 1;
                                }
                            } else {
                                nx = if dx == -1 { 4 } else { 0 };
                                for ny in 0..5 {
                                    neighbors[nz][ny as usize][nx as usize] += 1;
                                }
                            }
                        } else {
                            neighbors[nz][ny as usize][nx as usize] += 1;
                            if nz < min_z {
                                min_z = nz;
                            }
                        }
                    }
                }
            }
        }
        let new_grids_data = (min_z..=max_z)
            .map(|z| {
                (0..5)
                    .map(|y| {
                        (0..5)
                            .map(|x| {
                                let count = neighbors[z][y][x];
                                count == 1
                                    || ((z == 0
                                        || z > self.grids.len()
                                        || !self.grids[z - 1][y][x])
                                        && count == 2)
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        Self {
            grids: new_grids_data,
            lowest_level: if min_z == 0 {
                self.lowest_level - 1
            } else {
                self.lowest_level
            },
        }
    }

    fn display(&self) {
        for z in 0..self.grids.len() {
            println!("Depth {}:", self.lowest_level + z as isize);
            for y in 0..5 {
                for x in 0..5 {
                    if x == 2 && y == 2 {
                        print!("?");
                        continue;
                    }
                    print!("{}", if self.grids[z][y][x] { '#' } else { '.' });
                }
                println!();
            }
        }
        println!();
    }

    fn simulate(&self, steps: usize) -> Self {
        let mut recursive_grid = self.clone();
        for _ in 0..steps {
            recursive_grid = recursive_grid.step();
        }
        recursive_grid
    }

    fn count_bugs(&self) -> usize {
        self.grids
            .iter()
            .map(|grid| {
                grid.iter()
                    .map(|row| row.iter().map(|has_bug| *has_bug as usize).sum::<usize>())
                    .sum::<usize>()
            })
            .sum()
    }
}

pub fn run(input: &str) {
    let grid_data: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b == b'#').collect())
        .collect();
    let debug = false;

    let grid: Grid = Grid::new(&grid_data);
    let repeated_grid = grid.simulate_until_repetition();
    println!("{}", repeated_grid.biodiversity_rating());
    if debug {
        repeated_grid.display()
    }

    let recursive_grid = RecursiveGrid::new(&grid_data);
    let recursive_grid = recursive_grid.simulate(200);
    println!("{}", recursive_grid.count_bugs());
    if debug {
        recursive_grid.display()
    }
}
