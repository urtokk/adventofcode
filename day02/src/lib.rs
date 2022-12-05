// A,X 1 ROCK, B,Y 2 PAPER, C,Z 3 SCISSORS
pub fn part01(input: String) -> u32 {
    let mut points = 0;
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let a = chars.next().unwrap();
        chars.next();
        let b = chars.next().unwrap();

        match (&a, &b) {
            (&'A', &'Y') | (&'B', &'Z') | (&'C', &'X') => points += 6,
            (&'B', &'X') | (&'C', &'Y') | (&'A', &'Z') => {},
            _ => points += 3,
        }

        match b {
            'X' => points += 1,
            'Y' => points += 2,
            'Z' => points += 3,
            _ => (),
        }
    });
    points
}

// A,X 1 ROCK, B,Y 2 PAPER, C,Z 3 SCISSORS
// X Lose, Y Draw, Z Win
pub fn part02(input: String) -> u32 {
    let mut resulting_strategy = String::new();
    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let a = chars.next().unwrap();
        chars.next();
        let b = chars.next().unwrap();

        let our_action = action(&a, &b);
        let strategy = format!("{a} {our_action}\n");
        resulting_strategy.push_str(strategy.as_str());
    });

    part01(resulting_strategy)
}

fn action(a: &char, b: &char) -> char {
    match (a, b) {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        ('B', 'Y') => 'Y',
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn it_works() {
        let result = part01(INPUT.to_string());
        assert_eq!(result, 15);
    }
}
