advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Find starting position and direction
    let mut pos = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                pos = Some((i, j));
                break;
            }
        }
    }

    let mut visited = std::collections::HashSet::new();
    let (mut r, mut c) = pos.unwrap();
    let mut dir = 0; // 0=up, 1=right, 2=down, 3=left
    visited.insert((r, c));

    // Directions: up, right, down, left
    let dr = [-1, 0, 1, 0];
    let dc = [0, 1, 0, -1];

    loop {
        let next_r = r as i32 + dr[dir];
        let next_c = c as i32 + dc[dir];

        // Check if next position is out of bounds
        if next_r < 0 || next_r >= rows as i32 || next_c < 0 || next_c >= cols as i32 {
            break;
        }

        // Check if obstacle ahead
        if grid[next_r as usize][next_c as usize] == '#' {
            dir = (dir + 1) % 4; // Turn right
        } else {
            r = next_r as usize;
            c = next_c as usize;
            visited.insert((r, c));
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &c)| c == '^')
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    let mut count = 0;
    let dr = [-1, 0, 1, 0];
    let dc = [0, 1, 0, -1];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '.' && (r, c) != start_pos {
                grid[r][c] = '#';

                let mut pos = start_pos;
                let mut dir = 0;
                let mut visited = vec![vec![[false; 4]; cols]; rows];
                let mut steps = 0;

                loop {
                    // If we've seen this position+direction before and moved at least once, it's a loop
                    if visited[pos.0][pos.1][dir] && steps > 0 {
                        count += 1;
                        break;
                    }

                    visited[pos.0][pos.1][dir] = true;

                    let next_r = pos.0 as i32 + dr[dir];
                    let next_c = pos.1 as i32 + dc[dir];

                    // Exit if out of bounds
                    if next_r < 0 || next_r >= rows as i32 || next_c < 0 || next_c >= cols as i32 {
                        break;
                    }

                    // If we hit a wall, turn right
                    if grid[next_r as usize][next_c as usize] == '#' {
                        dir = (dir + 1) % 4;
                    } else {
                        // Otherwise move forward
                        pos = (next_r as usize, next_c as usize);
                        steps += 1;
                    }

                    // Prevent infinite loops
                    if steps > rows * cols * 4 {
                        break;
                    }
                }

                grid[r][c] = '.';
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
