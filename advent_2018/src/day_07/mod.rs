//! Solutions to 2018 day 7 problems
//! --- Day 7: The Sum of Its Parts ---
use lazy_static::lazy_static;
use regex::Regex;

/// Parse a step and its precondition from str
fn parse_line(input: &str) -> Result<(char, char), &'static str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"Step (\w) must be finished before step (\w) can begin."#).unwrap();
    }

    RE.captures(input)
        .map(|captures| {
            let parent = captures.get(1).unwrap().as_str().chars().next().unwrap();
            let child = captures.get(2).unwrap().as_str().chars().next().unwrap();
            (parent, child)
        })
        .ok_or("Failed to parse rule")
}


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
