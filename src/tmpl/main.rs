mod pt1;
mod pt2;

fn main() {
    let start = Instant::now();
    println!("Part 1:");
    pt1::pt1();
    println!("Part 1 done in '{:?}'", start.elapsed());

    let start = Instant::now();
    println!("Part 2:");
    pt2::pt2();
    println!("Part 2 done in '{:?}'", start.elapsed());
}
