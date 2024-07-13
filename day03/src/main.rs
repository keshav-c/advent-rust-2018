use day03;

fn main() {
    println!("Day 03 Part 1");
    let claims = day03::read_input("input.txt");
    let fabric = day03::process_claim(&claims);
    let uncontested_claims = day03::run2(&fabric);
    let overlapped_area = day03::run1(&fabric);
    println!(
        "The area of fabric with overlapping claims is {}",
        overlapped_area
    );
    println!(
        "The following claims are uncontested: {:?}",
        uncontested_claims
    );
}
