advent_of_code::solution!(1);

fn parse_input(input: &str) -> Option<(Vec<i64>, Vec<i64>)> {
    // Use as i64 to allow to calculate absolute value
    // and convert to u64 for summing later.
    let mut left_vec: Vec<i64> = Vec::new();
    let mut right_vec: Vec<i64> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() == 2 {
            left_vec.push(parts[0].parse().expect("Failed to parse integer"));
            right_vec.push(parts[1].parse().expect("Failed to parse integer"));
        }
        else {
            println!("Failed to parse line: {line}");
            std::process::exit(1);
        }
    }

    if left_vec.len() != right_vec.len() {
        println!("Vectors don't have equal length!");
        std::process::exit(1);

    }

    Some((left_vec, right_vec))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut l, mut r) = parse_input(input)?;

    l.sort();
    r.sort();

    let sum_of_difference: u64 = l.iter()
        .zip(r.iter())
        .map(|(a, b)| (*a - *b).abs() as u64)
        .sum();

    Some(sum_of_difference)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
