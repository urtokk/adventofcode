use std::fs;

fn main() {
    let input = fs::read_to_string("Input.txt").expect("Unable to read file").replace("\r\n", "\n");
    let result = day01::part_02(input);
    println!("Result: {}", result);
}