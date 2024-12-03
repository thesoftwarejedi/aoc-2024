use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().map(|line| { 
        let line = line.split_whitespace().collect_vec(); 
        (line[0].parse::<i32>().unwrap(), line[1].parse::<i32>().unwrap())
    }).collect_vec();
    let mut left: Vec<i32> = lines.iter().map(|(l, _)| *l).collect_vec();
    let mut right: Vec<i32> = lines.iter().map(|(_, r)| *r).collect_vec();

    left.sort();
    right.sort();

    let mut total_distance = 0i32;
    for (i, left_val) in left.iter().enumerate() {
        total_distance += (left_val - right[i]).abs();
    }

    Some(total_distance.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().map(|line| { 
        let line = line.split_whitespace().collect_vec(); 
        (line[0].parse::<i32>().unwrap(), line[1].parse::<i32>().unwrap())
    }).collect_vec();
    
    let left: Vec<i32> = lines.iter().map(|(l, _)| *l).collect_vec();
    let right: Vec<i32> = lines.iter().map(|(_, r)| *r).collect_vec();

    let similarity_score: i32 = left.iter()
        .map(|left_val| {
            // Count occurrences of left_val in right list
            let count = right.iter().filter(|&&r| r == *left_val).count() as i32;
            left_val * count
        })
        .sum();

    Some(similarity_score.try_into().unwrap())
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
        assert_eq!(result, None);
    }
}
