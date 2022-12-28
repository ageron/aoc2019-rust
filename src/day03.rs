use hashbrown::HashMap;
use regex::Regex;

type Path = Vec<(char, usize)>;

fn find_best_intersections(paths: &[Path]) -> (Option<usize>, Option<usize>) {
    let mut grid: HashMap<(isize, isize), (usize, usize)> = HashMap::new();
    let mut min_distance: Option<usize> = None;
    let mut min_combined_steps: Option<usize> = None;
    for (path_index, path) in paths.iter().enumerate() {
        let mut x = 0;
        let mut y = 0;
        let mut steps = 0;
        for (direction, length) in path {
            let (dx, dy) = match direction {
                'U' => (0, -1),
                'R' => (1, 0),
                'L' => (-1, 0),
                'D' => (0, 1),
                _ => unreachable!(),
            };
            for _ in 0..*length {
                x += dx;
                y += dy;
                steps += 1;
                let cell = grid.get(&(x, y));
                if let Some((old_path_index, old_steps)) = cell {
                    if *old_path_index == path_index {
                        continue;
                    } // path crossing itself
                    let distance = x.unsigned_abs() + y.unsigned_abs();
                    let min = min_distance.unwrap_or(usize::MAX);
                    if distance < min {
                        min_distance = Some(distance);
                    }
                    let combined_steps = *old_steps + steps;
                    let min = min_combined_steps.unwrap_or(usize::MAX);
                    if combined_steps < min {
                        min_combined_steps = Some(combined_steps);
                    }
                } else {
                    grid.insert((x, y), (path_index, steps));
                }
            }
        }
    }
    (min_distance, min_combined_steps)
}

pub fn run(input: &str) {
    let regex = Regex::new(r"(U|R|D|L)(\d+)").unwrap();
    let paths: Vec<Path> = input
        .lines()
        .map(|line| {
            regex
                .captures_iter(line)
                .map(|cap| {
                    (
                        cap[1].chars().next().unwrap(),
                        cap[2].parse::<usize>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();
    let (min_distance, min_combined_steps) = find_best_intersections(&paths);
    println!("{}", min_distance.unwrap());
    println!("{}", min_combined_steps.unwrap());
}
