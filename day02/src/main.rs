use day02;

fn main() {
    let input = day02::read_input("input.txt");
    let checksum = day02::calc_checksum(input);
    println!("Calculated checksum: {}", checksum);
}
