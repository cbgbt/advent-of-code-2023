const INPUT: &str = include_str!("input.dat");

fn main() {
    let result: u32 = INPUT
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));

            let first = digits.next().unwrap();
            first * 10 + digits.last().unwrap_or(first)
        })
        .sum();
    println!("{}", result);
}
