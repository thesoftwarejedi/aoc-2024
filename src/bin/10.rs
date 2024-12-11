use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(10);

// Direction vectors for up, right, down, left
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_one(input: &str) -> Option<u32> {
    // Parse the grid into a 2D vector
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // Return None if the grid is empty
    if grid.is_empty() || grid[0].is_empty() {
        return None;
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Find all trailheads (positions with height 0)
    let mut total_score = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == 0 {
                total_score += calculate_trailhead_score(&grid, (i, j));
            }
        }
    }

    Some(total_score)
}

fn calculate_trailhead_score(grid: &[Vec<u32>], start: (i32, i32)) -> u32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // Use BFS to find all reachable positions
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable_nines = HashSet::new();
    
    queue.push_back((start, 0)); // (position, current_height)
    visited.insert(start);

    while let Some(((row, col), height)) = queue.pop_front() {
        // If we reached a 9, add it to our set of reachable nines
        if grid[row as usize][col as usize] == 9 {
            reachable_nines.insert((row, col));
            continue;
        }

        // Check all four directions
        for (dr, dc) in DIRECTIONS {
            let new_row = row + dr;
            let new_col = col + dc;

            // Check bounds
            if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
                continue;
            }

            let new_pos = (new_row, new_col);
            let new_height = grid[new_row as usize][new_col as usize];

            // Only proceed if it's the next height and we haven't visited it
            if new_height == height + 1 && !visited.contains(&new_pos) {
                visited.insert(new_pos);
                queue.push_back((new_pos, new_height));
            }
        }
    }

    reachable_nines.len() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    if grid.is_empty() || grid[0].is_empty() {
        return None;
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Find all trailheads and calculate their ratings
    let mut total_rating = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == 0 {
                total_rating += calculate_trailhead_rating(&grid, (i, j));
            }
        }
    }

    Some(total_rating)
}

fn calculate_trailhead_rating(grid: &[Vec<u32>], start: (i32, i32)) -> u32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut memo = HashMap::new();
    
    fn dfs(
        pos: (i32, i32),
        grid: &[Vec<u32>],
        rows: i32,
        cols: i32,
        memo: &mut HashMap<(i32, i32), u32>,
    ) -> u32 {
        // If we've seen this position before, return its memoized value
        if let Some(&count) = memo.get(&pos) {
            return count;
        }

        let (row, col) = pos;
        let current_height = grid[row as usize][col as usize];
        
        // If we reached a 9, we found a valid path
        if current_height == 9 {
            return 1;
        }

        let mut total_paths = 0;
        
        // Try all four directions
        for (dr, dc) in DIRECTIONS {
            let new_row = row + dr;
            let new_col = col + dc;

            // Check bounds
            if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
                continue;
            }

            let new_height = grid[new_row as usize][new_col as usize];

            // Only proceed if it's exactly one higher
            if new_height == current_height + 1 {
                total_paths += dfs((new_row, new_col), grid, rows, cols, memo);
            }
        }

        // Memoize the result before returning
        memo.insert(pos, total_paths);
        total_paths
    }

    let paths = dfs(start, grid, rows, cols, &mut memo);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
