use hashbrown::HashMap;
use itertools::Itertools;
use pathfinding::directed::astar::astar;
use pathfinding::directed::dijkstra::dijkstra;

fn get_positions(maze: &[Vec<u8>]) -> HashMap<u8, (usize, usize)> {
    let mut positions = HashMap::new();
    for (y, row) in maze.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            positions.insert(*c, (x, y));
        }
    }
    positions
}

fn shortest_distance(
    maze: &[Vec<u8>],
    keys: &[u8],
    start: (usize, usize),
    target: (usize, usize),
) -> Option<usize> {
    let successors = |(x, y): &(usize, usize)| {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(dx, dy): (isize, isize)| (*x as isize + dx, *y as isize + dy))
            .filter(|(x, y): &(isize, isize)| {
                if *x < 0 || *y < 0 {
                    return false;
                }
                let (x, y) = (*x as usize, *y as usize);
                if y >= maze[0].len() || x >= maze[y].len() {
                    return false;
                }
                let c = maze[y][x];
                if c == b'#' {
                    return false;
                } // wall
                if (b'A'..=b'Z').contains(&c) && !keys.contains(&(c - b'A' + b'a')) {
                    return false;
                } // cannot open the door
                if (x, y) != target && (b'a'..=b'z').contains(&c) && !keys.contains(&c) {
                    return false;
                } // cannot pickup other key than target
                true
            })
            .map(|(x, y): (isize, isize)| {
                let (x, y) = (x as usize, y as usize);
                ((x, y), 1)
            })
            .collect::<Vec<_>>()
    };
    let heuristic = |(x, y): &(usize, usize)| {
        (target.0 as isize - *x as isize).unsigned_abs()
            + (target.1 as isize - *y as isize).unsigned_abs()
    };
    let success = |pos: &(usize, usize)| *pos == target;
    if let Some((_, total_cost)) = astar(&start, successors, heuristic, success) {
        Some(total_cost)
    } else {
        None
    }
}

fn shortest_distance_to_all_keys(maze: &mut [Vec<u8>], four_robots: bool) -> usize {
    let positions = get_positions(maze);
    let (start_x, start_y) = positions.get(&b'@').unwrap();
    let all_keys: Vec<u8> = positions
        .keys()
        .copied()
        .filter(|c| *c >= b'a' && *c <= b'z')
        .sorted()
        .collect();
    if four_robots {
        for (x, y) in [
            (*start_x - 1, *start_y),
            (*start_x + 1, *start_y),
            (*start_x, *start_y - 1),
            (*start_x, *start_y + 1),
        ] {
            maze[y][x] = b'#';
        }
    }
    let robots = if four_robots {
        vec![
            (start_x - 1, start_y - 1),
            (start_x - 1, start_y + 1),
            (start_x + 1, start_y - 1),
            (start_x + 1, start_y + 1),
        ]
    } else {
        vec![(*start_x, *start_y)]
    };
    let successors = |(robots, keys): &(Vec<(usize, usize)>, Vec<u8>)| {
        (b'a'..=b'z')
            .filter(|c| !keys.contains(c))
            .map(|c| {
                let (target_x, target_y) = positions[&c];
                let robot_index = if four_robots {
                    match (
                        (target_x as isize - *start_x as isize).signum(),
                        (target_y as isize - *start_y as isize).signum(),
                    ) {
                        (-1, -1) => 0,
                        (-1, 1) => 1,
                        (1, -1) => 2,
                        (1, 1) => 3,
                        _ => unreachable!(),
                    }
                } else {
                    0
                };
                let distance =
                    shortest_distance(maze, keys, robots[robot_index], (target_x, target_y));
                (distance, c, robot_index)
            })
            .filter(|(distance, _, _)| distance.is_some())
            .map(|(distance, c, robot_index)| {
                let mut keys = keys.clone();
                keys.push(c);
                keys.sort();
                let mut robots = robots.clone();
                robots[robot_index] = positions[&c];
                ((robots, keys), distance.unwrap())
            })
            .collect::<Vec<_>>()
    };
    let success = |(_, keys): &(Vec<(usize, usize)>, Vec<u8>)| *keys == all_keys;
    if let Some((_, total_cost)) = dijkstra(&(robots, vec![]), successors, success) {
        return total_cost;
    }
    unreachable!()
}

pub fn run(input: &str) {
    let mut maze: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let length = shortest_distance_to_all_keys(&mut maze, false);
    println!("{}", length);
    let length = shortest_distance_to_all_keys(&mut maze, true);
    println!("{}", length);
}
