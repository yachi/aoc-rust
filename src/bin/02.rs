advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut safe = 0;

    for line in input.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut slopes = vec![];

        let mut sf = true;
        for i in 0..(nums.len() - 1) {
            let slope = nums[i + 1] - nums[i];
            let sq = slope * slope;
            if !(1..10).contains(&sq) {
                println!("c: {} | {} {} | {}", line, nums[i + 1], nums[i], sq);
                sf = false;
            }
            if let Some(s) = slopes.last() {
                if s * slope < 0 {
                    println!("n: {} | {} {}", line, nums[i + 1], nums[i]);
                    sf = false;
                };
            }
            slopes.push(slope);
        }
        if sf {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut safe = 0;

    for line in input.lines() {
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if safe_ma(nums, 0) {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn safe_ma(nums: Vec<i64>, failed: i64) -> bool {
    let mut slopes = vec![];

    if failed == 0 {
        println!("==========");
    }
    println!("check: {:?}", nums);

    let mut sf = true;
    let mut sfi = 0;
    for i in 0..(nums.len() - 1) {
        let slope = nums[i + 1] - nums[i];
        let sq = slope * slope;
        if !(1..10).contains(&sq) {
            println!(
                "far: {:?} | {} {} | {}",
                nums,
                nums[i],
                nums[i + 1],
                (sq as f64).sqrt()
            );
            sf = false;
            sfi = i;
            break;
        }
        if let Some(s) = slopes.last() {
            if s * slope < 0 {
                println!("wd: {:?} | {} {}", nums, nums[i], nums[i + 1]);
                sf = false;
                sfi = i;
                break;
            };
        }
        slopes.push(slope);
    }

    println!("slopes: {:?}", slopes);

    if sf {
        println!("safe: {:?} | safe", nums);
    } else {
        println!("fail: {:?} | fail", nums);
    }

    if !sf && failed == 0 {
        let mut nums1 = nums.clone();
        let mut nums2 = nums.clone();
        nums1.remove(sfi);
        nums2.remove(sfi + 1);
        if safe_ma(nums1, 1) || safe_ma(nums2, 1) {
            return true;
        }
    }

    sf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
