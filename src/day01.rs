fn fuel(m: &isize) -> isize {
    m / 3 - 2
}

fn fuel_for_fuel(m: &isize) -> isize {
    let f = fuel(m);
    if f <= 0 {
        return 0;
    }
    f + fuel_for_fuel(&f)
}

pub fn run(input: &str) {
    let nums: Vec<isize> = input.lines().map(|x| x.parse::<isize>().unwrap()).collect();
    let fuel: isize = nums.iter().map(fuel).sum();
    println!("{}", fuel);
    let total_fuel: isize = nums.iter().map(fuel_for_fuel).sum();
    println!("{}", total_fuel);
}
