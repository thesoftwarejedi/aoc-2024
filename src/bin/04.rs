advent_of_code::solution!(4);

fn find_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (1, -1),  // diagonal down-left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, 1),  // diagonal up-right
        (-1, -1), // diagonal up-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for (dx, dy) in directions {
        let mut valid = true;
        let target = ['X', 'M', 'A', 'S'];

        for i in 0..4 {
            let new_row = (row as i32 + dx * i as i32) as i32;
            let new_col = (col as i32 + dy * i as i32) as i32;

            if new_row < 0
                || new_row >= rows as i32
                || new_col < 0
                || new_col >= cols as i32
                || grid[new_row as usize][new_col as usize] != target[i]
            {
                valid = false;
                break;
            }
        }

        if valid {
            count += 1;
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'X' {
                total += find_xmas(&grid, row, col);
            }
        }
    }

    Some(total)
}

fn find_xmas_part2(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    // Only check positions that are 'A' since it must be the center of the X
    if grid[row][col] != 'A' {
        return 0;
    }

    let diagonals = [
        [(1, -1), (-1, 1)], // bottom-left to top-right
        [(1, 1), (-1, -1)], // bottom-right to top-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check both diagonal pairs
    for diagonal_pair in diagonals {
        // Check if both diagonals form valid patterns
        let mut valid_diagonals = 0;

        // For each diagonal direction
        for &(dx, dy) in &diagonal_pair {
            // Check for M in one direction and S in the other
            let new_row_m = row as i32 - dx;
            let new_col_m = col as i32 - dy;
            let new_row_s = row as i32 + dx;
            let new_col_s = col as i32 + dy;

            if new_row_m >= 0
                && new_row_m < rows as i32
                && new_col_m >= 0
                && new_col_m < cols as i32
                && new_row_s >= 0
                && new_row_s < rows as i32
                && new_col_s >= 0
                && new_col_s < cols as i32
            {
                let m_char = grid[new_row_m as usize][new_col_m as usize];
                let s_char = grid[new_row_s as usize][new_col_s as usize];

                // Check both MAS and SAM patterns
                if (m_char == 'M' && s_char == 'S') || (m_char == 'S' && s_char == 'M') {
                    valid_diagonals += 1;
                }
            }
        }

        // If both diagonals in this pair are valid
        if valid_diagonals == 2 {
            count += 1;
        }
    }

    count / 2
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            total += find_xmas_part2(&grid, row, col);
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
