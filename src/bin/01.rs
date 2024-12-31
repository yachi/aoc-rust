use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut v1: Vec<u64> = vec![];
    let mut v2: Vec<u64> = vec![];

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        v1.push(nums[0].parse().unwrap());
        v2.push(nums[1].parse().unwrap());
    }
    v1.sort();
    v2.sort();

    let mut sum = 0;

    for (e1, e2) in v1.iter().zip(v2.iter()) {
        if e1 > e2 {
            sum = sum + e1 - e2;
        } else {
            sum = sum + e2 - e1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut v1: Vec<u64> = vec![];
    let mut v2: Vec<u64> = vec![];

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        v1.push(nums[0].parse().unwrap());
        v2.push(nums[1].parse().unwrap());
    }

    let mut sum = 0;

    let mut map = HashMap::new();

    for e in v2 {
        map.entry(e).and_modify(|x| *x += 1).or_insert(1);
    }

    for e in v1 {
        if let Some(v) = map.get(&e) {
            sum += e * v;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
