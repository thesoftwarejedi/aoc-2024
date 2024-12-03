advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            
            if numbers.len() < 2 {
                return false;
            }

            // Check if sequence is increasing or decreasing
            let mut increasing = true;
            let mut decreasing = true;
            
            for i in 1..numbers.len() {
                let diff = numbers[i] - numbers[i-1];
                
                // Check if difference is between 1 and 3
                if diff.abs() < 1 || diff.abs() > 3 {
                    return false;
                }
                
                if diff > 0 {
                    decreasing = false;
                } else {
                    increasing = false;
                }
                
                // If neither increasing nor decreasing, it's not safe
                if !increasing && !decreasing {
                    return false;
                }
            }
            
            true
        })
        .count() as u32;

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            
            if numbers.len() < 2 {
                return false;
            }

            // First check if it's already safe without removing any number
            if is_safe(&numbers) {
                return true;
            }

            // Try removing each number one at a time
            for skip_idx in 0..numbers.len() {
                let modified: Vec<i32> = numbers
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip_idx)
                    .map(|(_, &n)| n)
                    .collect();
                
                if is_safe(&modified) {
                    return true;
                }
            }
            
            false
        })
        .count() as u32;

    Some(safe_reports)
}

fn is_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    let mut increasing = true;
    let mut decreasing = true;
    
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i-1];
        
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        
        if diff > 0 {
            decreasing = false;
        } else {
            increasing = false;
        }
        
        if !increasing && !decreasing {
            return false;
        }
    }
    
    true
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
        assert_eq!(result, Some(4));
    }
}
