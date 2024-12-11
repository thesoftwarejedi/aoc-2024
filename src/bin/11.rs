advent_of_code::solution!(11);

use std::collections::HashMap;

fn count_stones_after_n_blinks(stones: Vec<u64>, n: usize) -> u64 {
    // Cache to store known results for (value, remaining_blinks) -> count
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();
    
    fn calculate_growth(stone: u64, blinks: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
        // Check cache first
        if let Some(&result) = cache.get(&(stone, blinks)) {
            return result;
        }
        
        if blinks == 0 {
            return 1;
        }

        let result = if stone == 0 {
            // 0 becomes 1, then follows rule 3 in next blinks
            calculate_growth(1, blinks - 1, cache)
        } else {
            let digits = stone.to_string();
            if digits.len() % 2 == 0 {
                // Even digits split into two numbers
                let mid = digits.len() / 2;
                let left: u64 = digits[..mid].parse().unwrap_or(0);
                let right: u64 = digits[mid..].parse().unwrap_or(0);
                calculate_growth(left, blinks - 1, cache) + 
                calculate_growth(right, blinks - 1, cache)
            } else {
                // Multiply by 2024
                calculate_growth(stone * 2024, blinks - 1, cache)
            }
        };

        cache.insert((stone, blinks), result);
        result
    }

    stones.iter()
        .map(|&stone| calculate_growth(stone, n, &mut cache))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(count_stones_after_n_blinks(stones, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(count_stones_after_n_blinks(stones, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("125 17");
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("125 17");
        assert_eq!(result, Some(131392));
    }
}
