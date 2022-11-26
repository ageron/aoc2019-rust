#[derive(Copy, Clone)]
enum Shuffle {
    DealIntoNewStack,
    DealWithIncrement(usize),
    Cut(usize),
    NegativeCut(usize),
}
use Shuffle::*;

impl Shuffle {
    fn new(line: &str) -> Self {
        if line == "deal into new stack" {
            return DealIntoNewStack;
        }
        let number_str = line.split(' ').last().unwrap();
        if line.starts_with("cut ") {
            let n: isize = number_str.parse().unwrap();
            if n > 0 {
                return Cut(n as usize);
            } else {
                return NegativeCut(-n as usize);
            }
        }
        DealWithIncrement(number_str.parse().unwrap())
    }
}

// I assume that module < 2^63 or else there is a risk of overflow
fn modulo_add(a: usize, b: usize, modulus: usize) -> usize {
    let a = a % modulus;
    let b = b % modulus;
    (a + b) % modulus
}

fn modulo_sub(a: usize, b: usize, modulus: usize) -> usize {
    let a = a % modulus;
    let b = b % modulus;
    (a + modulus - b) % modulus
}

fn modulo_mul(a: usize, b: usize, modulus: usize) -> usize {
    let mut a = a % modulus;
    let mut b = b % modulus;
    let mut res = 0;
    while a != 0 {
        if a % 2 == 1 {
            res = modulo_add(res, b, modulus);
        }
        a >>= 1;
        b = (b << 1) % modulus;
    }
    res
}

pub fn modulo_exp(a: usize, b: usize, modulus: usize) -> usize {
    let mut result = 1;
    let mut a = a % modulus;
    let mut b = b;
    loop {
        if b == 0 {
            return result;
        }
        if b % 2 == 1 {
            result = modulo_mul(result, a, modulus);
        }
        b >>= 1;
        a = modulo_mul(a, a, modulus);
    }
}

fn modulo_inv(a: usize, modulus: usize) -> usize {
    // assumes modulus is prime, or else this does not return the inverse
    modulo_exp(a, modulus - 2, modulus)
}

#[derive(Debug, Copy, Clone)]
struct Deck {
    // Represents a deck of cards numbered 0 to num_cards-1, then shuffled with
    // a deal_with_increment (multiplies the step between consecutive numbers),
    // followed by a cut (increments the offset), and possibly followed by a
    // deal_into_new_stack (reverses the order).
    num_cards: usize,
    reversed: bool,
    step: usize,
    offset: usize,
}

impl Deck {
    fn new(num_cards: usize) -> Self {
        Self {
            num_cards,
            step: 1,         // interval between consecutive numbers
            reversed: false, // false: left to right, true: right to left
            offset: 0,       // index of card 0
        }
    }

    fn apply(&mut self, shuffle: Shuffle) {
        match shuffle {
            DealWithIncrement(increment) => {
                self.step = modulo_mul(self.step, increment, self.num_cards);
                if self.reversed {
                    self.offset = self.num_cards - 1 - self.offset;
                }
                self.offset = modulo_mul(self.offset, increment, self.num_cards);
                if self.reversed {
                    self.offset = self.num_cards - 1 - self.offset;
                }
            }
            DealIntoNewStack => {
                self.reversed = !self.reversed;
            }
            _ => {
                let mut change = match shuffle {
                    Cut(index) => modulo_sub(self.num_cards, index, self.num_cards),
                    NegativeCut(index) => index,
                    _ => unreachable!(),
                };
                if self.reversed {
                    change = modulo_sub(0, change, self.num_cards);
                }
                self.offset = modulo_add(self.offset, change, self.num_cards);
            }
        }
    }

    fn index_of(&self, card: usize) -> usize {
        let index = modulo_add(
            modulo_mul(card, self.step, self.num_cards),
            self.offset,
            self.num_cards,
        );
        if self.reversed {
            self.num_cards - 1 - index
        } else {
            index
        }
    }

    fn value_at(&self, index: usize) -> usize {
        let index = if self.reversed {
            self.num_cards - 1 - index
        } else {
            index
        };
        let index = modulo_sub(index, self.offset, self.num_cards);
        modulo_mul(modulo_inv(self.step, self.num_cards), index, self.num_cards)
    }

    fn apply_shuffles(&mut self, shuffles: &[Shuffle]) {
        for shuffle in shuffles {
            self.apply(*shuffle);
        }
    }

    fn repeated(&mut self, repeats: usize) -> Deck {
        let repeats = repeats % self.num_cards;
        if repeats == 1 {
            return *self;
        }
        let shuffles = self.get_shuffles();
        let mut deck_twice = *self; // to cancel reverse, if any
        deck_twice.apply_shuffles(&shuffles);
        assert!(!deck_twice.reversed);
        let mut result = Deck::new(self.num_cards);
        result.reversed = false;
        result.step = modulo_exp(deck_twice.step, repeats / 2, self.num_cards);
        result.offset = modulo_mul(
            deck_twice.offset,
            modulo_sub(result.step, 1, self.num_cards),
            self.num_cards,
        );
        result.offset = modulo_mul(
            result.offset,
            modulo_inv(
                modulo_sub(deck_twice.step, 1, self.num_cards),
                self.num_cards,
            ),
            self.num_cards,
        );
        if repeats % 2 == 1 {
            result.apply_shuffles(&shuffles);
        }
        result
    }

    fn get_shuffles(&self) -> Vec<Shuffle> {
        let mut shuffles = vec![DealWithIncrement(self.step)];
        if self.offset > 0 {
            shuffles.push(Cut(self.num_cards - self.offset));
        }
        if self.reversed {
            shuffles.push(DealIntoNewStack);
        }
        shuffles
    }
}

pub fn run(input: &str) {
    let shuffles: Vec<_> = input.lines().map(Shuffle::new).collect();

    let num_cards: usize = 10007;
    let mut deck = Deck::new(num_cards);
    deck.apply_shuffles(&shuffles);
    println!("{}", deck.index_of(2019));

    let num_cards: usize = 119315717514047;
    let mut deck = Deck::new(num_cards);
    deck.apply_shuffles(&shuffles);
    let deck = deck.repeated(101741582076661);
    println!("{}", deck.value_at(2020));
}
