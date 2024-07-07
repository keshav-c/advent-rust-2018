use day02;

fn main() {
    let input = day02::read_input("input.txt");
    let checksum = day02::calc_checksum(&input);
    println!("Calculated checksum: {}", checksum);
    let common_id_letters = day02::find_the_common_id(&input).unwrap();
    println!("The common id letters are: {}", common_id_letters);
}
