advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    
    // Parse input and get dimensions
    for (y, line) in input.lines().enumerate() {
        width = line.len() as i32;
        height = y as i32 + 1;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();
    
    // For each frequency group
    for (_freq, positions) in antennas.iter() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let a1 = positions[i];
                let a2 = positions[j];
                
                // Vector from a1 to a2
                let dx = a2.x - a1.x;
                let dy = a2.y - a1.y;
                
                // Point that makes a2 twice as far as a1
                let antinode1 = Point {
                    x: a2.x + dx,
                    y: a2.y + dy,
                };
                
                // Point that makes a1 twice as far as a2
                let antinode2 = Point {
                    x: a1.x - dx,
                    y: a1.y - dy,
                };
                
                // Add antinodes if they're within bounds
                if antinode1.x >= 0 && antinode1.x < width && 
                   antinode1.y >= 0 && antinode1.y < height {
                    antinodes.insert(antinode1);
                }
                
                if antinode2.x >= 0 && antinode2.x < width && 
                   antinode2.y >= 0 && antinode2.y < height {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    
    // Parse input and get dimensions
    for (y, line) in input.lines().enumerate() {
        width = line.len() as i32;
        height = y as i32 + 1;
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();
    
    // For each frequency group
    for (_freq, positions) in antennas.iter() {
        // Check every point in the grid
        for y in 0..height {
            for x in 0..width {
                let point = Point { x, y };
                
                // Check if this point is collinear with any pair of antennas
                for i in 0..positions.len() {
                    for j in (i + 1)..positions.len() {
                        let a1 = positions[i];
                        let a2 = positions[j];
                        
                        if is_collinear(point, a1, a2) {
                            antinodes.insert(point);
                        }
                    }
                }
            }
        }
        
        // All antennas of the same frequency are also antinodes 
        // (unless they're the only antenna of that frequency)
        if positions.len() > 1 {
            for &pos in positions {
                antinodes.insert(pos);
            }
        }
    }

    Some(antinodes.len() as u32)
}

fn is_collinear(p: Point, a1: Point, a2: Point) -> bool {
    // Calculate cross product
    // If cross product is 0, points are collinear
    let dx1 = p.x - a1.x;
    let dy1 = p.y - a1.y;
    let dx2 = p.x - a2.x;
    let dy2 = p.y - a2.y;
    
    dx1 * dy2 == dx2 * dy1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
