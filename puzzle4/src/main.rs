extern crate itertools;

use itertools::Itertools;

fn verify_number(number: &i32) -> bool {
    let digits: Vec<u32> = number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let num_same: Vec<bool> = digits
        .iter()
        .tuple_windows()
        .filter_map(|(a, b)| if a == b { Some(*a) } else { None })
        .tuple_windows()
        .map(|(a, b)| a == b)
        .collect();

    if num_same.is_empty() || !num_same.iter().all(|&x| x) {
        return false;
    }

    digits
        .iter()
        .tuple_windows()
        .map(|(a, b)| if b >= a { 1 } else { 0 })
        .sum::<u32>()
        == 5
}

fn main() {
    let count = (172_930..=683_082).filter(verify_number).count();
    println!("Count: {}", count);
    println!(
        "Test: {} {} {}",
        verify_number(&689999),
        verify_number(&123789),
        verify_number(&223450)
    );
}
