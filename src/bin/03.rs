advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // Use regex to find valid mul(X,Y) patterns
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    let sum = re.captures_iter(input)
        .map(|cap| {
            let x: u32 = cap[1].parse().unwrap();
            let y: u32 = cap[2].parse().unwrap();
            x * y
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Match both control instructions and multiplication operations
    let re = regex::Regex::new(r"(?:do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    
    let mut enabled = true;
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let matched = cap.get(0).unwrap().as_str();
        match matched {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            mul_expr => {
                if enabled {
                    // If we're here, we matched a mul expression and have captures
                    let x: u32 = cap[1].parse().unwrap();
                    let y: u32 = cap[2].parse().unwrap();
                    sum += x * y;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(test_input), Some(161));
    }

    #[test]
    fn test_part_two() {
        let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(part_two(test_input), Some(48));
    }
}
