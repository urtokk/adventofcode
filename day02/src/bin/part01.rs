fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", day02::part01(input));
}