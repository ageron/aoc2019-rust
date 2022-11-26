use super::intcode::{Program, ProgramState::*};
struct Springdroid {
    brain: Program,
    video_feed: bool,
    last_char: u8,
    final_output: isize,
}

impl Springdroid {
    fn new(intcode: &[isize], video_feed: bool) -> Self {
        Self {
            brain: Program::new(intcode),
            video_feed,
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
                print!("{}", val as char);
                self.last_char = val;
            }
        }
    }

    fn send_instructions(&mut self, instructions: &[&str]) -> isize {
        while self.brain.state() == Running {
            self.step();
        }
        for instruction in instructions {
            if self.video_feed {
                println!("{}", instruction);
            }
            for c in instruction.bytes() {
                self.brain.send(c as isize);
            }
            self.brain.send(b'\n' as isize);
            self.step();
            while self.brain.state() == Running {
                self.step();
            }
        }
        self.final_output
    }
}

fn shortsighted_jumps(intcode: &[isize], video_feed: bool) -> isize {
    let mut springdroid = Springdroid::new(intcode, video_feed);
    let instructions = vec![
        //(NOT A OR NOT B OR NOT C) AND D
        "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "NOT D T", "NOT T T", "AND T J", "WALK",
    ];
    springdroid.send_instructions(&instructions)
}

fn farsighted_jumps(intcode: &[isize], video_feed: bool) -> isize {
    let mut springdroid = Springdroid::new(intcode, video_feed);
    let instructions = vec![
        //D AND (E OR H) AND NOT (A AND B AND C))
        "NOT A T", "NOT T T", "AND B T", "AND C T", "NOT T T", "NOT E J", "NOT J J", "OR H J",
        "AND T J", "AND D J", "RUN",
    ];
    springdroid.send_instructions(&instructions)
}

pub fn run(input: &str) {
    let intcode: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let video_feed = false;
    let output = shortsighted_jumps(&intcode, video_feed);
    println!("{}", output);
    let video_feed = false;
    let output = farsighted_jumps(&intcode, video_feed);
    println!("{}", output);
}
