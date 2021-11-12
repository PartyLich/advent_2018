//! Solutions to 2018 day 7 problems part two
//! --- Day 7: The Sum of Its Parts ---

/// get the duration for a step with the supplied label
fn get_time(ch: char) -> usize {
    ch as usize - 64 + 60
}

/// returns the completion time (in seconds) of the assembly steps
pub fn two(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_time() {
        let msg = "should return the duration for a given step label";
        let expected = 61;
        let actual = get_time('A');
        assert_eq!(actual, expected, "{}", msg);

        let expected = 86;
        let actual = get_time('Z');
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the total assembly time in seconds";
        let expected = 15;
        let actual = two("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
