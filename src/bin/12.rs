use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant_type = grid[r][c];
                let (area, perimeter) = calculate_region(&grid, &mut visited, r, c, plant_type);
                total_price += area * perimeter;
            }
        }
    }

    Some(total_price)
}

fn calculate_region(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    plant_type: char,
) -> (u32, u32) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut stack = vec![(row, col)];
    let rows = grid.len();
    let cols = grid[0].len();

    while let Some((r, c)) = stack.pop() {
        if visited[r][c] || grid[r][c] != plant_type {
            continue;
        }

        visited[r][c] = true;
        area += 1;

        // Check all four sides for perimeter calculation
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in directions {
            let new_r = r as i32 + dr;
            let new_c = c as i32 + dc;

            if new_r >= 0 && new_r < rows as i32 && new_c >= 0 && new_c < cols as i32 {
                let nr = new_r as usize;
                let nc = new_c as usize;
                if grid[nr][nc] == plant_type {
                    stack.push((nr, nc));
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}

/**
 * THIS IS WRONG. I won't go any further.
 */
pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut total_price = 0;

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant_type = grid[r][c];
                let (area, sides) = calculate_region_sides(&grid, &mut visited, r, c, plant_type);
                total_price += area * sides;
            }
        }
    }

    Some(total_price)
}

fn calculate_region_sides(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    plant_type: char,
) -> (u32, u32) {
    let mut area = 0;
    let mut sides = HashSet::new();
    let mut stack = vec![(row, col)];
    let rows = grid.len();
    let cols = grid[0].len();

    while let Some((r, c)) = stack.pop() {
        if visited[r][c] || grid[r][c] != plant_type {
            continue;
        }

        visited[r][c] = true;
        area += 1;

        // Check all four sides
        // Top
        if r == 0 || grid[r - 1][c] != plant_type {
            sides.insert((r, c, 0));
        }
        // Right
        if c + 1 >= cols || grid[r][c + 1] != plant_type {
            sides.insert((r, c, 1));
        }
        // Bottom
        if r + 1 >= rows || grid[r + 1][c] != plant_type {
            sides.insert((r, c, 2));
        }
        // Left
        if c == 0 || grid[r][c - 1] != plant_type {
            sides.insert((r, c, 3));
        }

        // Add unvisited neighbors
        if c + 1 < cols && grid[r][c + 1] == plant_type {
            stack.push((r, c + 1));
        }
        if r + 1 < rows && grid[r + 1][c] == plant_type {
            stack.push((r + 1, c));
        }
        if c > 0 && grid[r][c - 1] == plant_type {
            stack.push((r, c - 1));
        }
        if r > 0 && grid[r - 1][c] == plant_type {
            stack.push((r - 1, c));
        }
    }

    (area, sides.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
