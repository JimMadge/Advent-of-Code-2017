pub mod day3 {
    use std::cmp::min;

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum Direction {
        Right,
        Up,
        Left,
        Down,
    }

    impl Direction {
        pub fn next_direction(&self) -> Direction {
            match self {
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
            }
        }
    }

    pub fn manhattan_distance(coordinate: (i32, i32)) -> i32 {
        coordinate.0.abs() + coordinate.1.abs()
    }

    pub fn spiral_coordinates(address: u32) -> (i32, i32) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut direction = Direction::Right;
        let mut span: u32 = 1;
        let mut second: bool = false;
        let mut remaining = address-1;

        loop {
            // Return if at address
            if remaining == 0 {
                return (x,y)
            }

            // Calculate how far to move
            let change = min(span, remaining) as i32;

            // Move in set direction
            if direction == Direction::Right {
                x += change;
            } else if direction == Direction::Up {
                y += change;
            } else if direction == Direction::Left {
                x -= change;
            } else if direction == Direction::Down {
                y -= change;
            }
            remaining -= change as u32;

            // Change direction
            direction = direction.next_direction();

            // Increment span every second direction
            if second {
                span += 1;
                second = false;
            } else {
                second = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day3::Direction;
    use super::day3::manhattan_distance;
    use super::day3::spiral_coordinates;

    #[test]
    fn test_next_direction_1() {
        assert_eq!(Direction::Up, Direction::Right.next_direction());
    }

    #[test]
    fn test_next_direction_2() {
        assert_eq!(Direction::Left, Direction::Up.next_direction());
    }

    #[test]
    fn test_next_direction_3() {
        assert_eq!(Direction::Down, Direction::Left.next_direction());
    }

    #[test]
    fn test_next_direction_4() {
        assert_eq!(Direction::Right, Direction::Down.next_direction());
    }

    #[test]
    fn test_manhatten_distance_1() {
        assert_eq!(2, manhattan_distance((1,1)));
    }

    #[test]
    fn test_manhatten_distance_2() {
        assert_eq!(3, manhattan_distance((-2,1)));
    }

    #[test]
    fn test_manhatten_distance_3() {
        assert_eq!(4, manhattan_distance((-2,-2)));
    }

    #[test]
    fn test_spiral_coordinates_1() {
        assert_eq!((0,0), spiral_coordinates(1));
    }

    #[test]
    fn test_spiral_coordinates_2() {
        assert_eq!((1,0), spiral_coordinates(2));
    }

    #[test]
    fn test_spiral_coordinates_3() {
        assert_eq!((-2,-2), spiral_coordinates(21));
    }

    #[test]
    fn test_spiral_coordinates_4() {
        assert_eq!((-2,2), spiral_coordinates(17));
    }
}
