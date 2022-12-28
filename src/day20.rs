use hashbrown::HashMap;
use pathfinding::directed::bfs::bfs;

type Portals = HashMap<(usize, usize), (usize, usize)>;
type NamedLocations = HashMap<String, (usize, usize)>;

fn get_portals_and_named_locations(maze: &[Vec<u8>]) -> (Portals, NamedLocations) {
    let mut portals = HashMap::new();
    let mut named_locations = HashMap::new();
    for (y, row) in maze.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if (b'A'..=b'Z').contains(c) {
                let mut c2 = 0;
                let mut pos = (0, 0);
                if x < row.len() - 1 && (b'A'..=b'Z').contains(&row[x + 1]) {
                    c2 = row[x + 1];
                    if x > 0 && row[x - 1] == b'.' {
                        pos = (x - 1, y);
                    } else {
                        pos = (x + 2, y)
                    }
                } else if y < maze.len() - 1 && (b'A'..=b'Z').contains(&maze[y + 1][x]) {
                    c2 = maze[y + 1][x];
                    if y > 0 && maze[y - 1][x] == b'.' {
                        pos = (x, y - 1);
                    } else {
                        pos = (x, y + 2)
                    }
                }
                if c2 != 0 {
                    let label = String::from_utf8(vec![*c, c2]).unwrap();
                    let other_pos = named_locations.get(&label);
                    if let Some(other_pos) = other_pos {
                        portals.insert(pos, *other_pos);
                        portals.insert(*other_pos, pos);
                        named_locations.insert([label, "2".to_string()].join(""), pos);
                    } else {
                        named_locations.insert(label, pos);
                    }
                }
            }
        }
    }
    (portals, named_locations)
}

fn is_outer_portal(maze: &[Vec<u8>], x: usize, y: usize) -> bool {
    y < 3 || y > maze.len() - 1 - 3 || x < 3 || x > maze[0].len() - 1 - 3
}

fn shortest_distance(maze: &[Vec<u8>], is_recursive: bool) -> usize {
    let (portals, named_locations) = get_portals_and_named_locations(maze);
    let start = named_locations.get("AA").unwrap();
    let start = (start.0, start.1, 0); // start at level 0
    let target = named_locations.get("ZZ").unwrap();
    let target = (target.0, target.1, 0);

    let successors = |(x, y, level): &(usize, usize, usize)| -> Vec<(usize, usize, usize)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(dx, dy)| (*x as isize + dx, *y as isize + dy))
            .filter(|(x, y)| {
                if *x < 0 || *y < 0 {
                    return false;
                }
                let (x, y) = (*x as usize, *y as usize);
                if y >= maze.len() || x >= maze[y].len() {
                    return false;
                }
                maze[y][x] == b'.'
            })
            .map(|(x, y)| (x as usize, y as usize, *level))
            .chain({
                if let Some((nx, ny)) = portals.get(&(*x, *y)) {
                    if is_recursive {
                        if is_outer_portal(maze, *x, *y) {
                            if *level > 0 {
                                vec![(*nx, *ny, *level - 1)].into_iter()
                            } else {
                                vec![].into_iter() // cannot go beyond outermost layer
                            }
                        } else {
                            vec![(*nx, *ny, *level + 1)].into_iter()
                        }
                    } else {
                        vec![(*nx, *ny, *level)].into_iter()
                    }
                } else {
                    vec![].into_iter() // not a portal location
                }
            })
            .collect()
    };
    let success = |(x, y, level): &(usize, usize, usize)| (*x, *y, *level) == target;
    bfs(&start, successors, success).unwrap().len() - 1
}

pub fn run(input: &str) {
    let maze: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let distance = shortest_distance(&maze, false);
    println!("{}", distance);
    let distance = shortest_distance(&maze, true);
    println!("{}", distance);
}
