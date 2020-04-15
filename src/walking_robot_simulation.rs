/*
874. Walking Robot Simulation

A robot on an infinite grid starts at point (0, 0) and faces north.  The robot can receive one of three possible types of commands:

-2: turn left 90 degrees
-1: turn right 90 degrees
1 <= x <= 9: move forward x units
Some of the grid squares are obstacles.

The i-th obstacle is at grid point (obstacles[i][0], obstacles[i][1])

If the robot would try to move onto them, the robot stays on the previous grid square instead (but still continues following the rest of the route.)

Return the square of the maximum Euclidean distance that the robot will be from the origin.


Example 1:
Input: commands = [4,-1,3], obstacles = []
Output: 25
Explanation: robot will go to (3, 4)

Example 2:
Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
Output: 65
Explanation: robot will be stuck at (1, 4) before turning left and going to (1, 8)

Note:
0 <= commands.length <= 10000
0 <= obstacles.length <= 10000
-30000 <= obstacle[i][0] <= 30000
-30000 <= obstacle[i][1] <= 30000
The answer is guaranteed to be less than 2 ^ 31.
*/

use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solution;

#[derive(Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}


impl Solution {
    fn next_point(p: &Point, direction: &Direction) -> Point {
        match direction {
            Direction::North => Point { x: p.x, y: p.y + 1 },
            Direction::East => Point { x: p.x + 1, y: p.y },
            Direction::South => Point { x: p.x, y: p.y - 1 },
            Direction::West => Point { x: p.x - 1, y: p.y },
        }
    }

    fn robot_move(p: Point, direction: &Direction, distance: i32, obstacles: &HashSet<Point>) -> Point {
        let mut current_position = p;
        for _ in 0..distance {
            let next_candidate = Self::next_point(&current_position, &direction);
            if obstacles.contains(&next_candidate) {
                break;
            }
            current_position = next_candidate;
        }

        current_position
    }

    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut direction = Direction::North;
        let mut position = Point { x: 0, y: 0 };
        let mut max_distance = 0i32;

        let obstacles: HashSet<Point> = HashSet::from_iter(obstacles.iter().cloned().map(|p| Point { x: p[0], y: p[1] }));

        for command in commands {
            match command {
                -2 => direction = Direction::turn_left(&direction),
                -1 => direction = Direction::turn_right(&direction),
                x => position = Self::robot_move(position, &direction, x, &obstacles),
            }

            let distance = position.x * position.x + position.y * position.y;
            max_distance = std::cmp::max(max_distance, distance);
        }

        max_distance
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
        assert_eq!(Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]), 65);
    }
}