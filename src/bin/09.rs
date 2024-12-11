advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse_input(input);
    compact_disk(&mut blocks);
    Some(calculate_checksum(&blocks))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks = parse_input(input);
    compact_disk_whole_files(&mut blocks);
    Some(calculate_checksum(&blocks))
}

fn parse_input(input: &str) -> Vec<Option<u64>> {
    let mut blocks = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    for c in input.trim().chars() {
        let count = match c.to_digit(10) {
            Some(n) => n as u64,
            None => {
                println!("Warning: Skipping invalid character: {}", c);
                continue;
            }
        };

        if is_file {
            for _ in 0..count {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..count {
                blocks.push(None);
            }
        }
        is_file = !is_file;
    }

    blocks
}

fn compact_disk(blocks: &mut Vec<Option<u64>>) {
    let len = blocks.len();
    
    for i in (0..len).rev() {
        if let Some(file_id) = blocks[i] {
            let mut target = 0;
            while target < i && blocks[target].is_some() {
                target += 1;
            }
            
            if target < i {
                blocks[target] = Some(file_id);
                blocks[i] = None;
            }
        }
    }
}

fn compact_disk_whole_files(blocks: &mut Vec<Option<u64>>) {
    // Find the highest file ID
    let max_file_id = blocks.iter()
        .filter_map(|&x| x)
        .max()
        .unwrap_or(0);
    
    // Process files from highest ID to lowest
    for file_id in (0..=max_file_id).rev() {
        // Count the file size first
        let file_size = blocks.iter()
            .filter(|&&x| x == Some(file_id))
            .count();
        
        if file_size == 0 {
            continue;
        }

        // Find the current start position of the file
        let current_start = blocks.iter()
            .position(|&x| x == Some(file_id))
            .unwrap();

        // Find the leftmost valid position
        let mut best_position = None;
        let mut i = 0;
        while i < current_start {
            // Check if we have enough contiguous space starting at position i
            let mut space_available = 0;
            let mut j = i;
            while j < current_start && blocks[j].is_none() {
                space_available += 1;
                if space_available >= file_size {
                    best_position = Some(i);
                    break;
                }
                j += 1;
            }
            if best_position.is_some() {
                break;
            }
            i = j + 1;
        }

        // Move the file if we found a better position
        if let Some(new_start) = best_position {
            // Clear the old position
            for i in current_start..current_start + file_size {
                blocks[i] = None;
            }
            // Place at new position
            for i in 0..file_size {
                blocks[new_start + i] = Some(file_id);
            }
        }
    }
}

fn calculate_checksum(blocks: &[Option<u64>]) -> u64 {
    blocks.iter().enumerate().filter_map(|(i, &block)| {
        block.map(|file_id| (i as u64) * file_id)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("2333133121414131402");
        assert_eq!(result, Some(1928));
    }
 
    #[test]
    fn test_part_two() {
        let result = part_two("2333133121414131402");
        assert_eq!(result, Some(2858));
    }
}
