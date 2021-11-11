//! Solutions to 2018 day 7 problems
//! --- Day 7: The Sum of Its Parts ---

/// returns the order of the assembly steps
pub fn one(file_path: &str) -> String {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the order of the assembly steps";
        let expected = "CABDFE".to_string();
        let actual = one("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
