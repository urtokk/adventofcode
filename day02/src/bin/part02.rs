fn main() {
    let input = std::fs::read_to_string("input02.txt").unwrap();
    println!("{}", day02::part02(input));
}