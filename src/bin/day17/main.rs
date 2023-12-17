use std::time::Instant;

mod bothparts;

fn main() {
    let start = Instant::now();
    println!("Part 1:");
    bothparts::pt1();
    println!("Part 1 done in '{:?}'", start.elapsed());

    let start = Instant::now();
    println!("Part 2:");
    bothparts::pt2();
    println!("Part 2 done in '{:?}'", start.elapsed());
}
