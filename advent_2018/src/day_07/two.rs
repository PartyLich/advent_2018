//! Solutions to 2018 day 7 problems part two
//! --- Day 7: The Sum of Its Parts ---

/// returns the completion time (in seconds) of the assembly steps
pub fn two(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should return the total assembly time in seconds";
        let expected = 15;
        let actual = two("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
