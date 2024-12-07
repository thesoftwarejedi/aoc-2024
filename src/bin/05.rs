advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    // Split input into rules and updates sections
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = parts[0].lines();
    let updates = parts[1].lines();

    // Parse rules into (before, after) pairs
    let rule_pairs: Vec<(u32, u32)> = rules
        .filter(|line| !line.is_empty())
        .map(|rule| {
            let nums: Vec<u32> = rule.split('|').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    // Process each update
    let mut middle_sum = 0;

    'updates: for update in updates {
        if update.is_empty() {
            continue;
        }

        // Parse update numbers
        let nums: Vec<u32> = update.split(',').map(|n| n.parse().unwrap()).collect();

        // Check all applicable rules
        for (before, after) in &rule_pairs {
            if nums.contains(before) && nums.contains(after) {
                let pos_before = nums.iter().position(|&x| x == *before).unwrap();
                let pos_after = nums.iter().position(|&x| x == *after).unwrap();
                if pos_before > pos_after {
                    continue 'updates; // Invalid order
                }
            }
        }

        // Valid update - add middle number to sum
        middle_sum += nums[nums.len() / 2];
    }

    Some(middle_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = parts[0].lines();
    let updates = parts[1].lines();

    // Parse rules into (before, after) pairs
    let rule_pairs: Vec<(u32, u32)> = rules
        .filter(|line| !line.is_empty())
        .map(|rule| {
            let nums: Vec<u32> = rule.split('|').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    let mut middle_sum = 0;

    for update in updates {
        if update.is_empty() {
            continue;
        }

        // Parse update numbers
        let mut nums: Vec<u32> = update.split(',').map(|n| n.parse().unwrap()).collect();

        // Check if update is invalid
        let mut is_valid = true;
        for (before, after) in &rule_pairs {
            if nums.contains(before) && nums.contains(after) {
                let pos_before = nums.iter().position(|&x| x == *before).unwrap();
                let pos_after = nums.iter().position(|&x| x == *after).unwrap();
                if pos_before > pos_after {
                    is_valid = false;
                    break;
                }
            }
        }

        if !is_valid {
            // Sort the invalid update
            nums.sort_by(|a, b| {
                // If there's a rule a|b, a comes first
                // If there's a rule b|a, b comes first
                // Otherwise maintain current order
                for (before, after) in &rule_pairs {
                    if *before == *a && *after == *b {
                        return std::cmp::Ordering::Less;
                    }
                    if *before == *b && *after == *a {
                        return std::cmp::Ordering::Greater;
                    }
                }
                std::cmp::Ordering::Equal
            });

            middle_sum += nums[nums.len() / 2];
        }
    }

    Some(middle_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
