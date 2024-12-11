advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
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
            sum += target;
        }
    }

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
                break; // Skip this combination if we've already exceeded target
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

        if can_make_value_part2_fast(target, &numbers) {
            sum += target;
        }
    }

    Some(sum)
}

fn can_make_value_part2_fast(target: u64, numbers: &[u64]) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }

    // Use a stack-based approach instead of recursion
    let mut stack = Vec::new();
    stack.push((numbers[0], 1)); // (current_value, next_index)

    while let Some((value, idx)) = stack.pop() {
        if idx == numbers.len() {
            if value == target {
                return true;
            }
            continue;
        }

        let next = numbers[idx];

        // Try addition
        if let Some(sum) = value.checked_add(next) {
            if sum <= target {
                stack.push((sum, idx + 1));
            }
        }

        // Try multiplication
        if let Some(product) = value.checked_mul(next) {
            if product <= target {
                stack.push((product, idx + 1));
            }
        }

        // Try concatenation
        if let Some(concat) = concat_numbers(value, next) {
            if concat <= target {
                stack.push((concat, idx + 1));
            }
        }
    }

    false
}

#[inline]
fn concat_numbers(a: u64, b: u64) -> Option<u64> {
    let b_digits = count_digits(b);
    let shift = 10_u64.pow(b_digits as u32);
    a.checked_mul(shift).and_then(|x| x.checked_add(b))
}

#[inline]
fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
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
        assert_eq!(result, Some(424977609625985));
    }
}
