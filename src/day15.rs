use crate::intcode::Program;
use itertools::Itertools;
use pathfinding::directed::bfs::bfs;
use std::collections::HashSet;

struct Robot {
    x: isize,
    y: isize,
    brain: Program,
}

impl Robot {
    fn new(intcode: &[isize]) -> Self {
        Self {
            x: 0,
            y: 0,
            brain: Program::new(intcode),
        }
    }

    fn maybe_move(&mut self, direction: isize) -> isize {
        self.brain.send(direction);
        while self.brain.num_outputs() == 0 {
            self.brain.step();
        }
        let output = self.brain.receive().unwrap();
        let index = (direction - 1) as usize;
        if output != 0 {
            self.x += [0, 0, -1, 1][index];
            self.y += [-1, 1, 0, 0][index];
        }
        output
    }

    fn map_spaceship(&mut self) -> (HashSet<(isize, isize)>, (isize, isize)) {
        let mut map = HashSet::new();
        let mut stack: Vec<isize> = vec![];
        let reverse_directions = [0, 2, 1, 4, 3];
        let mut oxygen_location = (0, 0);
        'outer: loop {
            map.insert((self.x, self.y));
            for direction in 1..=4 {
                let result = self.maybe_move(direction);
                if result != 0 {
                    if result == 2 {
                        oxygen_location = (self.x, self.y);
                    }
                    let reverse_dir = reverse_directions[direction as usize];
                    if map.contains(&(self.x, self.y)) {
                        self.maybe_move(reverse_dir);
                    } else {
                        stack.push(reverse_dir);
                        continue 'outer;
                    }
                }
            }
            if let Some(reverse_dir) = stack.pop() {
                self.maybe_move(reverse_dir);
            } else {
                return (map, oxygen_location);
            }
        }
    }
}

fn display(map: &HashSet<(isize, isize)>, oxygen_location: (isize, isize)) {
    let (min_x, max_x) = map.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    for y in (*min_y - 1)..=(*max_y + 1) {
        for x in (*min_x - 1)..=(*max_x + 1) {
            if x == 0 && y == 0 {
                print!("R");
            } else if oxygen_location == (x, y) {
                print!("X");
            } else if map.contains(&(x, y)) {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!();
    }
}

fn shortest_path_length(
    map: &HashSet<(isize, isize)>,
    start: (isize, isize),
    target: (isize, isize),
) -> usize {
    let successors = |(x, y): &(isize, isize)| {
        [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(x, y)| map.contains(&(*x, *y)))
            .collect::<Vec<_>>()
    };
    bfs(&start, successors, |pos: &(isize, isize)| *pos == target)
        .unwrap()
        .len()
        - 1
}

fn longest_path(map: &HashSet<(isize, isize)>, start: (isize, isize)) -> usize {
    let mut to_visit: HashSet<(isize, isize)> = HashSet::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut n_steps = 0;
    to_visit.insert(start);
    while !to_visit.is_empty() {
        let mut next_to_visit: HashSet<(isize, isize)> = HashSet::new();
        n_steps += 1;
        for (x, y) in &to_visit {
            visited.insert((*x, *y));
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if visited.contains(&(nx, ny))
                    || to_visit.contains(&(nx, ny))
                    || next_to_visit.contains(&(nx, ny))
                    || !map.contains(&(nx, ny))
                {
                    continue;
                }
                next_to_visit.insert((nx, ny));
            }
        }
        to_visit = next_to_visit;
    }
    n_steps - 1
}

pub fn run(input: &str) {
    let intcode: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let mut robot = Robot::new(&intcode);
    let (map, oxygen_location) = robot.map_spaceship();
    if false {
        display(&map, oxygen_location);
    }
    let n_steps = shortest_path_length(&map, (0, 0), oxygen_location);
    println!("{}", n_steps);
    let n_minutes = longest_path(&map, oxygen_location);
    println!("{}", n_minutes);
}
