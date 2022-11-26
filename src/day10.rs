use std::cmp::Ordering;
use std::collections::HashSet;

pub fn gcd(a: usize, b: usize) -> usize {
    if a == b {
        return a;
    }
    let mut a = a;
    let mut b = b;
    loop {
        if a < b {
            b %= a;
            if b == 0 {
                return a;
            }
        } else {
            a %= b;
            if a == 0 {
                return b;
            }
        }
    }
}

fn step(dx: isize, dy: isize) -> (isize, isize) {
    if dx == 0 {
        return (0, dy.signum());
    }
    if dy == 0 {
        return (dx.signum(), 0);
    }
    let gcd_ = gcd(dx.unsigned_abs(), dy.unsigned_abs()) as isize;
    (dx / gcd_, dy / gcd_)
}

fn count_detectable(asteroids: &[(isize, isize)], x: isize, y: isize) -> usize {
    let steps: HashSet<(isize, isize)> = asteroids
        .iter()
        .map(|(ox, oy)| step(ox - x, oy - y))
        .collect();
    steps.len() - 1
}

fn rotation_order_v2(dx1: isize, dy1: isize, dx2: isize, dy2: isize) -> Ordering {
    if step(dx1, dy1) == step(dx2, dy2) {
        // aligned
        return (dx1.abs() + dy1.abs()).cmp(&(dx2.abs() + dy2.abs())); // closest first
    }
    let angle1 = (dx1 as f64).atan2(dy1 as f64);
    let angle2 = (dx2 as f64).atan2(dy2 as f64);
    if angle1 < angle2 {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn laser_vaporize(asteroids: &[(isize, isize)], index_max: usize, target: usize) -> (isize, isize) {
    let (base_x, base_y) = asteroids[index_max];
    let mut asteroids: Vec<(isize, isize)> = asteroids
        .iter()
        .map(|(ox, oy)| (*ox - base_x, *oy - base_y))
        .filter(|(ox, oy)| (*ox != 0 || *oy != 0))
        .collect();
    asteroids.sort_by(|p1, p2| rotation_order_v2(p1.0, p1.1, p2.0, p2.1));
    let mut count = 0;
    let mut last_step = (0, 0);
    let mut index = 0;
    loop {
        let (x, y) = asteroids[index];
        let s = step(x, y);
        if s == last_step {
            // hidden by asteroid just destroyed
            index = (index + 1) % asteroids.len();
            if index == 0 {
                last_step = (0, 0);
            }
            continue;
        }
        count += 1;
        if count == target {
            let (dx, dy) = asteroids[index];
            return (base_x + dx, base_y + dy);
        }
        asteroids.remove(index);
        last_step = s;
    }
}

pub fn run(input: &str) {
    let asteroids: Vec<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|(_, c)| *c == b'#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();
    let (index_max, max_detectable) = asteroids
        .iter()
        .map(|(x, y)| count_detectable(&asteroids, *x, *y))
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .unwrap();
    println!("{}", max_detectable);
    let asteroid = laser_vaporize(&asteroids, index_max, 200);
    println!("{}", asteroid.0 * 100 + asteroid.1);
}
