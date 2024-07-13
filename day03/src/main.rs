use day03;

fn main() {
    println!("Day 03 Part 1");
    let claims = day03::read_input("input.txt");
    let overlapped_area = day03::run1(&claims);
    println!(
        "The area of fabric with overlapping claims is {}",
        overlapped_area
    );
}
