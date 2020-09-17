pub mod day5 {
    use std::fs;

    pub fn read_instructions(filename: &str) -> Vec<i32> {
        let input = fs::read_to_string(filename)
            .expect("unable to read file");

        let mut instructions: Vec<i32>  = Vec::new();
        for line in input.lines() {
            instructions.push(line.parse().unwrap());
        }
        instructions
    }

    pub fn count_steps(mut instructions: Vec<i32>) -> i32 {
        let mut steps = 0;

        let max_index: usize = instructions.len() - 1;
        let min_index: usize = 0;

        let mut index: usize = 0;

        loop {
            // Return if out of bounds
            if index < min_index || index > max_index {
                return steps
            }

            // Execute one step
            let jump = instructions[index] as isize;
            instructions[index] += 1;
            if jump >= 0 {
                index += jump.abs() as usize;
            } else {
                index -= jump.abs() as usize;
            }

            steps += 1;
        }
    }

    pub fn count_steps2(mut instructions: Vec<i32>) -> i32 {
        let mut steps = 0;

        let max_index: usize = instructions.len() - 1;
        let min_index: usize = 0;

        let mut index: usize = 0;

        loop {
            // Return if out of bounds
            if index < min_index || index > max_index {
                return steps
            }

            // Execute one step
            let jump = instructions[index] as isize;
            if jump >= 3 {
                instructions[index] -= 1;
            } else {
                instructions[index] += 1;
            }
            if jump >= 0 {
                index += jump.abs() as usize;
            } else {
                index -= jump.abs() as usize;
            }

            steps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::day5::count_steps;
    use super::day5::count_steps2;

    #[test]
    fn test_count_steps() {
        assert_eq!(count_steps(vec![0, 3, 0, 1, -3]), 5);
    }

    #[test]
    fn test_count_steps2() {
        assert_eq!(count_steps2(vec![0, 3, 0, 1, -3]), 10);
    }
}
