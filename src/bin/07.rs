advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    let mut count = 0;
    let mut max_target = 0;
    let mut max_numbers = 0;
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: u64 = parts[0].trim().parse().unwrap();
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        
        max_target = max_target.max(target);
        max_numbers = max_numbers.max(numbers.len());
        
        if can_make_value(target, &numbers) {
            count += 1;
            sum += target;
        }
    }
    
    println!("\nStatistics:");
    println!("Number of valid equations: {}", count);
    println!("Largest target value: {}", max_target);
    println!("Max numbers in an equation: {}", max_numbers);
    println!("Final sum: {}", sum);
    
    Some(sum)
}

fn can_make_value(target: u64, numbers: &[u64]) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }
    
    let ops_needed = numbers.len() - 1;
    let combinations = 1 << ops_needed;
    
    for i in 0..combinations {
        let mut result = numbers[0];
        
        for pos in 0..ops_needed {
            let use_multiply = (i >> pos) & 1 == 1;
            let next_num = numbers[pos + 1];
            
            if use_multiply {
                result *= next_num;
            } else {
                result += next_num;
            }
            
            // Check for overflow
            if result > target && use_multiply {
                break;  // Skip this combination if we've already exceeded target
            }
        }
        
        if result == target {
            return true;
        }
    }
    
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: u64 = parts[0].trim().parse().unwrap();
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
            
        if can_make_value_part2(target, &numbers) {
            sum += target;
        }
    }
    
    Some(sum)
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Concat
}

fn can_make_value_part2(target: u64, numbers: &[u64]) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }
    
    let ops_needed = numbers.len() - 1;
    let combinations = 1 << (2 * ops_needed); // 2 bits per operator
    
    'outer: for i in 0..combinations {
        let mut result = numbers[0];
        
        for pos in 0..ops_needed {
            let op = match (i >> (2 * pos)) & 3 {
                0 => Op::Add,
                1 => Op::Mul,
                2 => Op::Concat,
                _ => Op::Add
            };
            
            let next = numbers[pos + 1];
            
            result = match op {
                Op::Add => {
                    if let Some(r) = result.checked_add(next) {
                        r
                    } else {
                        continue 'outer;
                    }
                },
                Op::Mul => {
                    if let Some(r) = result.checked_mul(next) {
                        r
                    } else {
                        continue 'outer;
                    }
                },
                Op::Concat => {
                    let s = format!("{}{}", result, next);
                    if s.len() <= 20 {
                        match s.parse::<u64>() {
                            Ok(n) => n,
                            Err(_) => continue 'outer,
                        }
                    } else {
                        continue 'outer;
                    }
                }
            };
            
            if result > target {
                continue 'outer;
            }
        }
        
        if result == target {
            return true;
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(11387));
    }
}
