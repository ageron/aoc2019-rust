use hashbrown::HashMap;
use itertools::Itertools;

use super::intcode::{Program, ProgramState::*};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,  // 0 is an empty tile. No game object appears in this tile.
    Wall,   // 1 is a wall tile. Walls are indestructible barriers.
    Block,  // 2 is a block tile. Blocks can be broken by the ball.
    Paddle, // 3 is a horizontal paddle tile. The paddle is indestructible.
    Ball,   // 4 is a ball tile. The ball moves diagonally and bounces off objects.
}

use Tile::*;

impl Tile {
    fn new(value: isize) -> Self {
        match value {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => unreachable!(),
        }
    }

    fn to_char(self) -> char {
        match self {
            Empty => ' ',
            Wall => '█',
            Block => '#',
            Paddle => '_',
            Ball => 'o',
        }
    }
}

fn display_game(screen: &HashMap<(isize, isize), Tile>, score: usize) {
    let (min_x, max_x) = screen
        .keys()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = screen
        .keys()
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();
    print!("\x1b[1;1H");
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            let tile = *screen.get(&(x, y)).unwrap_or(&Empty);
            print!("{}", tile.to_char());
        }
        println!();
    }
    println!("Score: {}", score);
    println!();
}

fn run_game(intcode: &[isize], beat_game: bool, display: bool) -> usize {
    let mut game = Program::new(intcode);
    if beat_game {
        game.write(0, 2); // no need for quarters
    }
    let mut screen: HashMap<(isize, isize), Tile> = HashMap::new();
    let mut num_blocks: usize = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;
    while game.state() != Exited {
        game.step();
        while game.num_outputs() >= 3 {
            let x = game.receive().unwrap();
            let y = game.receive().unwrap();
            if (x, y) == (-1, 0) {
                score = game.receive().unwrap() as usize;
            } else {
                if let Some(tile) = screen.get(&(x, y)) {
                    if *tile == Block {
                        num_blocks -= 1;
                    }
                }
                let new_tile = Tile::new(game.receive().unwrap());
                match new_tile {
                    Block => {
                        num_blocks += 1;
                    }
                    Ball => {
                        ball_x = x;
                    }
                    Paddle => {
                        paddle_x = x;
                    }
                    _ => {}
                }
                screen.insert((x, y), new_tile);
            }
        }
        if game.state() == WaitingForInput {
            game.send((ball_x - paddle_x).signum());
        }
        if display && (game.state() == WaitingForInput || game.state() == Exited) {
            display_game(&screen, score);
        }
    }
    if beat_game {
        score
    } else {
        num_blocks
    }
}

pub fn run(input: &str) {
    let intcode: Vec<isize> = input
        .split(',')
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let num_blocks = run_game(&intcode, false, false);
    println!("{}", num_blocks);
    let score = run_game(&intcode, true, false);
    println!("{}", score);
}
