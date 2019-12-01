use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn fuel_for_mass(x: u64) -> u64 {
    std::iter::successors(Some(x), |x| Some((x / 3).saturating_sub(2)))
        .skip(1)
        .take_while(|&x| x > 0)
        .sum()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let fuel: u64 = reader
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse().ok())
        .map(fuel_for_mass)
        .sum::<u64>();

    println!("Fuel: {}", fuel);

    Ok(())
}
