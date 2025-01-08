use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0_u64;
    let re = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    for cap in re.captures_iter(input) {
        let left: u64 = cap.name("left").unwrap().as_str().parse().unwrap();
        let right: u64 = cap.name("right").unwrap().as_str().parse().unwrap();
        sum += left * right;
    }

    Some(sum)
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
        assert_eq!(result, Some(161_u64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
