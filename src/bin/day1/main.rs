const INPUT: &str = include_str!("input.dat");

fn pt1() {
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

fn parse_digit(inp: &str) -> u32 {
    match inp {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => inp.parse().unwrap(),
    }
}

fn pt2() {
    let digit_re =
        regex::Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let result: u32 = INPUT
        .lines()
        .map(|line| {
            let mut digits = (0..line.len()).filter_map(|i| {
                digit_re
                    .captures(&line[i..])
                    .map(|capture| parse_digit(&capture[1]))
            });

            let first = digits.next().unwrap();
            first * 10 + digits.last().unwrap_or(first)
        })
        .sum();

    println!("{}", result);
}

fn main() {
    println!("Part 1:");
    pt1();
    println!("Part 2:");
    pt2();
}
