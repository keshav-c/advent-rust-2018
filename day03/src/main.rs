use day03;

fn main() {
    println!("Day 03 Part 1");
    let overlapped_area = day03::run1("input.txt");
    println!(
        "The area of fabric with overlapping claims is {}",
        overlapped_area
    );
}
