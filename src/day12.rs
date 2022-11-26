use itertools::Itertools;
use regex::Regex;

fn lcm(a: usize, b: usize) -> usize {
    a * b / super::day10::gcd(a, b)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self {
            pos: [x, y, z],
            vel: [0, 0, 0],
        }
    }

    fn potential_energy(&self) -> usize {
        self.pos.iter().map(|value| value.unsigned_abs()).sum()
    }

    fn kinetic_energy(&self) -> usize {
        self.vel.iter().map(|value| value.unsigned_abs()).sum()
    }

    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }

    fn run_simulation(moons: &mut [Moon], steps: usize, check_repetition: bool) -> usize {
        let start_position: Vec<Moon> = moons.to_vec();
        for step in 1..=steps {
            for moon1 in 0..moons.len() {
                for moon2 in 0..moons.len() {
                    for axis in 0..3 {
                        moons[moon1].vel[axis] +=
                            (moons[moon2].pos[axis] - moons[moon1].pos[axis]).signum();
                    }
                }
            }
            for moon in moons.iter_mut() {
                for axis in 0..3 {
                    moon.pos[axis] += moon.vel[axis];
                }
            }
            if check_repetition && moons == start_position {
                return step;
            }
        }
        assert!(!check_repetition);
        moons.iter().map(|moon| moon.total_energy()).sum()
    }

    fn filter_axis(&self, axis: usize) -> Self {
        let mut moon = *self;
        for i in 0..3 {
            if i == axis {
                continue;
            }
            moon.pos[i] = 0;
            moon.vel[i] = 0;
        }
        moon
    }

    fn steps_to_repeat(moons: &[Moon]) -> usize {
        let steps: Vec<usize> = (0..3)
            .map(|axis| {
                let mut axis_moons: Vec<Moon> =
                    moons.iter().map(|moon| moon.filter_axis(axis)).collect();
                Moon::run_simulation(&mut axis_moons, usize::MAX, true)
            })
            .collect();
        lcm(lcm(steps[0], steps[1]), steps[2])
    }
}

pub fn run(input: &str) {
    let regex = Regex::new(r"(-?\d+)").unwrap();
    let moons: Vec<Moon> = input
        .lines()
        .map(|line| {
            regex
                .captures_iter(line)
                .map(|cap| cap[1].parse::<isize>().unwrap())
                .collect_tuple::<(isize, isize, isize)>()
                .unwrap()
        })
        .map(|(x, y, z)| Moon::new(x, y, z))
        .collect();
    let total_energy = Moon::run_simulation(&mut moons.clone(), 1000, false);
    println!("{}", total_energy);
    let total_steps = Moon::steps_to_repeat(&moons);
    println!("{}", total_steps);
}
