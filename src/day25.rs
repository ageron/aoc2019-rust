use crate::intcode::{Program, ProgramState::*};
use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead};

struct Droid {
    brain: Program,
    echo: bool,
}

impl Droid {
    fn new(intcode: &[isize], echo: bool) -> Self {
        Self {
            brain: Program::new(intcode),
            echo,
        }
    }

    fn run_until_next_command(&mut self) -> String {
        let mut outputs = vec![];
        loop {
            self.brain.step();
            while let Some(output) = self.brain.receive() {
                let output = output as u8;
                if self.echo {
                    print!("{}", output as char);
                }
                outputs.push(output);
            }
            if self.brain.state() != Running {
                break;
            }
        }
        String::from_utf8(outputs).unwrap()
    }

    fn send_command(&mut self, command: &str) {
        for b in command.bytes() {
            self.brain.send(b as isize);
        }
        self.brain.send(b'\n' as isize);
    }
}

pub fn run(input: &str) {
    let intcode: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let manual = false;
    let debug = false;
    let mut droid = Droid::new(&intcode, debug);

    if manual {
        let stdin = io::stdin();
        loop {
            droid.run_until_next_command();
            let command = stdin.lock().lines().next().unwrap().unwrap();
            droid.send_command(&command);
        }
    } else {
        let commands = [
            "south",
            "take fixed point",
            "north",
            "north",
            "take candy cane",
            "west",
            "take antenna",
            "south",
            "take whirled peas",
            "north",
            "west",
            "take shell",
            "east",
            "east",
            "north",
            "north",
            "take polygon",
            "south",
            "west",
            "take fuel cell",
            "west",
        ];
        for command in commands {
            droid.run_until_next_command();
            droid.send_command(command);
        }

        let items = [
            "shell",
            "whirled peas",
            "fuel cell",
            "fixed point",
            "polygon",
            "antenna",
            "candy cane",
        ];

        let regex = Regex::new(
            r"You should be able to get in by typing (.+) on the keypad at the main airlock",
        )
        .unwrap();

        'outer: for length in 0..=items.len() {
            for combination in items.iter().combinations(length) {
                for item in &combination {
                    droid.run_until_next_command();
                    droid.send_command(&format!("drop {}", item));
                }
                droid.run_until_next_command();
                droid.send_command("west");
                for item in &combination {
                    let output = droid.run_until_next_command();
                    if let Some(cap) = regex.captures_iter(&output).next() {
                        println!("{}", &cap[1]);
                        break 'outer;
                    }
                    droid.send_command(&format!("take {}", item));
                }
            }
        }
    }
}
