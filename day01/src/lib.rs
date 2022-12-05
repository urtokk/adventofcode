pub fn part_01(input: String) -> u32 {
    let vec_sums = prepare_vector(input);

    vec_sums.iter().max().unwrap().clone()
}

pub fn part_02(input: String) -> u32 {
    let mut vec_sums = prepare_vector(input);

    vec_sums
    .sort_by(|a, b| b.cmp(a));

    vec_sums.iter().take(3).sum()
}

fn prepare_vector(input: String) -> Vec<u32> {
        input
        .split("\n\n")
        .map(|load| {
            load
                .lines()
                .map(|line| line.parse::<u32>())
                .filter(|line| line.is_ok())
                .map(|line| line.unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_01() {
        let result = part_01(INPUT.to_string());
        assert_eq!(result, 24_000);
    }

    #[test]
    fn test_part_02() {
        let result = part_02(INPUT.to_string());
        assert_eq!(result, 45_000);
    }
}
