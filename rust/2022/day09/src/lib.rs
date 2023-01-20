use std::collections::HashSet;

pub mod input;

/// Simple clonable structure to hold the coordinates of knots
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

struct Rope {
    knots: Vec<Point<isize>>,
    unique_tail_positions: HashSet<Point<isize>>,
}

impl Rope {
    pub fn new(knots: usize) -> Self {
        // Preallocate all knots in the knots array, all at position x=0 y=0
        Self {
            knots: (0..knots).map(|_| Point::new(0, 0)).collect(),
            unique_tail_positions: HashSet::from([Point::new(0, 0)]),
        }
    }

    pub fn get_unique_tail_positions_count(&self) -> usize {
        self.unique_tail_positions.len()
    }

    fn up(&mut self) {
        self.knots[0].y += 1;
        self.update_knots();
    }

    fn down(&mut self) {
        self.knots[0].y -= 1;
        self.update_knots();
    }

    fn left(&mut self) {
        self.knots[0].x -= 1;
        self.update_knots();
    }

    fn right(&mut self) {
        self.knots[0].x += 1;
        self.update_knots();
    }

    fn update_knots(&mut self) {
        let knots_len = self.knots.len();
        for next_knot_index in 1..knots_len {
            let previous_knot = self.knots[next_knot_index - 1];
            let mut current_knot = &mut self.knots[next_knot_index];

            let x_spread = previous_knot.x - current_knot.x;
            let y_spread = previous_knot.y - current_knot.y;

            if isize::abs(x_spread) > 1 || isize::abs(y_spread) > 1 {
                /*
                    Head and tail are not adjacent at this point. Tail will need to move.
                    If they're not adjacent, one of the spreads is 2 and the other is either 0 or 1.
                    If it's 0, we're moving in one direction, either top, right, bottom or left. If it's
                    1, then we have to move diagonally, meaning moving both x and y by a combination of
                    -1 and/or 1.
                 */
                current_knot.x += x_spread.signum();
                current_knot.y += y_spread.signum();

                if next_knot_index + 1 == knots_len {
                    // Only store the unique position of the tail/last knot
                    self.unique_tail_positions.insert(current_knot.clone());
                }
            } else {
                // If the current knot didn't move, the ones behind it certainly won't either
                break;
            }
        }
    }
}

pub enum Command {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
}

pub mod parse_command_text {
    use super::*;

    pub fn parse_line(line: &str) -> Command {
        let (command, n) = line.split_once(' ')
            .expect("Expect there to be a space separating the command from the number of occurrences");
        let n = n.parse().expect("Expect number of occurrences to be a valid number");
        match command {
            "U" => Command::UP(n),
            "D" => Command::DOWN(n),
            "L" => Command::LEFT(n),
            "R" => Command::RIGHT(n),
            _ => panic!("Unknown command")
        }
    }
}

fn count_unique_tail_positions(lines: impl Iterator<Item=String>, knots: usize) -> usize {
    let mut rope = Rope::new(knots);
    for line in lines {
        match parse_command_text::parse_line(&line) {
            Command::UP(n) => for _ in 0..n { rope.up() }
            Command::DOWN(n) => for _ in 0..n { rope.down() }
            Command::LEFT(n) => for _ in 0..n { rope.left() }
            Command::RIGHT(n) => for _ in 0..n { rope.right() }
        }
    }
    rope.get_unique_tail_positions_count()
}

/// Response to the first part
pub fn count_unique_tail_positions_with_2_knots(lines: impl Iterator<Item=String>) -> usize {
    count_unique_tail_positions(lines, 2)
}

/// Response to the second part
pub fn count_unique_tail_positions_with_10_knots(lines: impl Iterator<Item=String>) -> usize {
    count_unique_tail_positions(lines, 10)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_unique_tail_positions_2_knots() {
        assert_eq!(count_unique_tail_positions_with_2_knots("\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2".lines().map(String::from)), 13);
    }

    #[test]
    fn test_count_unique_tail_positions_10_knots() {
        assert_eq!(count_unique_tail_positions_with_10_knots("\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2".lines().map(String::from)), 1);
    }

    #[test]
    fn test_count_unique_tail_positions_10_knots_larger_input() {
        assert_eq!(count_unique_tail_positions_with_10_knots("\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20".lines().map(String::from)), 36);
    }
}