use crate::intcode::Program;

fn in_beam(intcode: &[isize], x: usize, y: usize) -> bool {
    let mut program = Program::new(intcode);
    program.send(x as isize);
    program.send(y as isize);
    while program.num_outputs() == 0 {
        program.step()
    }
    let output = program.receive().unwrap();
    assert!(output == 0 || output == 1);
    output == 1
}

fn count_in_beam_50x50(intcode: &[isize], display: bool) -> usize {
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            if in_beam(intcode, x, y) {
                if display {
                    print!("#");
                }
                count += 1;
            } else if display {
                print!(".");
            }
        }
        if display {
            println!();
        }
    }
    count
}

fn closest_100x100_in_beam(intcode: &[isize]) -> usize {
    let (mut x, mut y) = (0, 0);
    loop {
        if in_beam(intcode, x + 100 - 1, y) {
            // top right
            if in_beam(intcode, x, y + 100 - 1) {
                // lower left
                return x * 10000 + y;
            } else {
                x += 1;
            }
        } else {
            y += 1;
        }
    }
}

pub fn run(input: &str) {
    let intcode: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let display = false;
    let count = count_in_beam_50x50(&intcode, display);
    println!("{}", count);
    let location = closest_100x100_in_beam(&intcode);
    println!("{}", location);
}
