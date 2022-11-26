use itertools::Itertools;

use super::intcode::{Program, ProgramState::*};
use std::collections::HashSet;

struct AsciiRobot {
    brain: Program,
    video_feed: bool,
    ansi_terminal: bool,
    next_home: bool,
    last_char: u8,
    final_output: isize,
}

fn ansi_terminal_home() {
    print!("\x1b[1;1H"); // ANSI Code to jump to top left corner
}

fn ansi_terminal_clear() {
    print!("\x1b[2J"); // ANSI Code to jump to top left corner
    ansi_terminal_home();
}

impl AsciiRobot {
    fn new(intcode: &[isize], video_feed: bool, ansi_terminal: bool) -> Self {
        Self {
            brain: Program::new(intcode),
            video_feed,
            ansi_terminal,
            next_home: true,
            last_char: 0,
            final_output: 0,
        }
    }

    fn step(&mut self) {
        self.brain.step();
        while self.brain.num_outputs() > 0 {
            let val = self.brain.receive().unwrap();
            if val > 127 {
                self.final_output = val;
            } else if self.video_feed {
                let val = val as u8;
                if self.ansi_terminal {
                    if self.last_char == b'\n' && val == b'\n' {
                        self.next_home = true;
                    } else {
                        if self.next_home {
                            ansi_terminal_home();
                            self.next_home = false;
                        }
                        print!("{}", val as char);
                    }
                } else {
                    print!("{}", val as char);
                }
                self.last_char = val;
            }
        }
    }
}

fn get_map(intcode: &[isize]) -> Vec<Vec<u8>> {
    let mut robot = Program::new(intcode);
    while robot.state() == Running {
        robot.step();
    }
    let mut data: Vec<u8> = vec![];
    while robot.num_outputs() > 0 {
        let c = robot.receive().unwrap() as u8;
        data.push(c);
    }
    String::from_utf8(data)
        .unwrap()
        .lines()
        .map(|s| s.bytes().collect())
        .collect()
}

fn is_robot(map: &[Vec<u8>], x: isize, y: isize) -> bool {
    matches!(get_terrain(map, x, y), b'<' | b'^' | b'>' | b'v')
}

fn is_scaffold(map: &[Vec<u8>], x: isize, y: isize) -> bool {
    get_terrain(map, x, y) == b'#' || is_robot(map, x, y)
}

fn intersection_alignment_parameters(map: &[Vec<u8>]) -> usize {
    let mut total = 0;
    for y in 1..(map.len() - 1) as isize {
        'next_position: for x in 1..(map[0].len() - 1) as isize {
            for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
                if !is_scaffold(map, x + dx, y + dy) {
                    continue 'next_position;
                }
            }
            total += x * y;
        }
    }
    total as usize
}

fn get_terrain(map: &[Vec<u8>], x: isize, y: isize) -> u8 {
    if x < 0 || y < 0 {
        return b'.';
    }
    let x = x as usize;
    let y = y as usize;
    if y >= map.len() || x >= map[y].len() {
        return b'.';
    }
    map[y][x]
}

fn get_path(map: &[Vec<u8>]) -> Vec<(bool, usize)> {
    let mut x = 0;
    let mut y = 0;
    'find_robot: for ry in 0..(map.len() as isize) {
        for rx in 0..(map[0].len() as isize) {
            if is_robot(map, rx, ry) {
                x = rx;
                y = ry;
                break 'find_robot;
            }
        }
    }
    let (mut dx, mut dy) = match get_terrain(map, x, y) {
        b'^' => (0, -1),
        b'v' => (0, 1),
        b'<' => (-1, 0),
        b'>' => (1, 0),
        _ => unreachable!(),
    };
    let mut path: Vec<(bool, usize)> = vec![];
    let mut length = 0;
    let mut is_left = true;
    loop {
        if is_scaffold(map, x + dx, y + dy) {
            x += dx;
            y += dy;
            length += 1;
        } else {
            if length > 0 {
                path.push((is_left, length));
            }
            if is_scaffold(map, x + dy, y - dx) {
                is_left = true;
                (dx, dy) = (dy, -dx);
            } else if is_scaffold(map, x - dy, y + dx) {
                is_left = false;
                (dx, dy) = (-dy, dx);
            } else {
                return path;
            }
            length = 0;
        }
    }
}

fn path_to_str(path: &[(bool, usize)]) -> String {
    let mut output: Vec<String> = vec![];
    for (is_left, length) in path.iter() {
        let dir = if *is_left { 'L' } else { 'R' };
        output.push(format!("{},{}", dir, length));
    }
    output.join(",")
}

fn recursive_search(
    sequences: &[Vec<(bool, usize)>],
    path: &[(bool, usize)],
    index: usize,
    main: &[usize],
    functions: &[usize],
) -> Option<String> {
    let remaining_length = path.len() - index;
    if remaining_length == 0 {
        let main = main
            .iter()
            .map(|idx| ["A", "B", "C"][*idx].to_string())
            .join(",");
        let functions = functions
            .iter()
            .map(|idx| path_to_str(&sequences[*idx]))
            .join("\n");
        return Some([main, functions].join("\n"));
    }
    if main.len() == 10 {
        return None;
    } // main is too long
    for (seq_index, seq) in sequences.iter().enumerate() {
        if seq.len() > remaining_length {
            continue;
        } // sequence is too long
        if &path[index..index + seq.len()] != seq {
            continue;
        } // sequence is not right
        let mut new_functions = functions.to_vec();
        let func_index;
        if let Some(idx) = functions.iter().position(|i| *i == seq_index) {
            func_index = idx;
        } else {
            if functions.len() == 3 {
                continue;
            } // all 3 functions already taken
            new_functions.push(seq_index);
            func_index = functions.len();
        }
        let mut new_main = main.to_vec();
        new_main.push(func_index);
        let solution = recursive_search(
            sequences,
            path,
            index + seq.len(),
            &new_main,
            &new_functions,
        );
        if solution.is_some() {
            return solution;
        } // solution found!
    }
    None
}

fn find_routines(map: &[Vec<u8>]) -> String {
    let path = get_path(map);
    let mut sequences: HashSet<Vec<(bool, usize)>> = HashSet::new();
    for start_index in 0..path.len() {
        for length in 1..=(path.len() - start_index) {
            let seq = path[start_index..start_index + length].to_vec();
            let seq_str = path_to_str(&seq);
            if seq_str.len() > 20 {
                continue;
            }
            sequences.insert(seq);
        }
    }
    let sequences: Vec<Vec<(bool, usize)>> = sequences
        .into_iter()
        .sorted_by_key(|seq| seq.len())
        .rev()
        .collect();
    recursive_search(&sequences, &path, 0, &[], &[]).unwrap()
}

fn save_robots(robot: &mut AsciiRobot, map: &[Vec<u8>]) -> usize {
    robot.brain.write(0, 2); // wake up
    let routines = find_routines(map);
    let mut answers: Vec<_> = routines.lines().collect();
    answers.push(if robot.video_feed { "y" } else { "n" });

    for answer in answers {
        while robot.brain.state() == Running {
            robot.step();
        } // wait for prompt
        if robot.video_feed {
            println!("{}", answer);
        }
        for b in answer.bytes() {
            robot.brain.send(b as isize);
        } // send answer
        robot.brain.send(b'\n' as isize); // line feed
        robot.step();
    }

    if robot.video_feed {
        ansi_terminal_clear();
    }
    while robot.brain.state() == Running {
        robot.step();
    }
    if robot.video_feed {
        println!();
    }
    robot.final_output as usize
}

pub fn run(input: &str) {
    let intcode: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let map = get_map(&intcode);
    let params = intersection_alignment_parameters(&map);
    println!("{}", params);

    let video_feed = false;
    let ansi_terminal = false;
    let mut robot = AsciiRobot::new(&intcode, video_feed, ansi_terminal);
    let dust_amount = save_robots(&mut robot, &map);
    println!("{}", dust_amount);
}
