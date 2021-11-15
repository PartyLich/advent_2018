#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
//! Advent of Code 2018 Solutions
use std::{fmt, fs, ops::RangeInclusive, path::Path};

use parser::three::lib::{any_of, choice, keep_first, p_char, p_int, spaces};

pub mod day_07;

/// read the specified file at `file_path` into a `String`
///
/// Panic! on error
pub fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    fs::read_to_string(&path).unwrap_or_else(|why| panic!("couldnt open {}: {}", display, why))
}

/// AoC problem solver function pointer
pub type Solver<T> = fn(&str) -> T;

/// AoC problem solution
pub struct Solution<T: fmt::Display> {
    /// Input filename
    pub input: &'static str,
    /// Part one output label and solving fn
    pub one: Option<(&'static str, Solver<T>)>,
    /// Part two output label and solving fn
    pub two: Option<(&'static str, Solver<T>)>,
}

impl<T: fmt::Display> std::fmt::Debug for Solution<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solution")
            .field("input file", &self.input)
            .finish()
    }
}

/// format as [`Solver`] with the supplied label and solving fn
#[macro_export]
macro_rules! to_solver {
    ($label: literal, $fn: path) => {
        Some(($label, |input| $fn(input).to_string()))
    };
    ($label: expr, $fn: path) => {
        Some(($label, |input| $fn(input).to_string()))
    };
}

/// format as [`Solution`]
#[macro_export]
macro_rules! to_solution {
    ($file: literal, ($fn_one: path,$text_one: literal), ($fn_two: path,$text_two: literal)) => {
        Solution {
            input: $file,
            one: to_solver!($text_one, $fn_one),
            two: to_solver!($text_two, $fn_two),
        }
    };
    ($file: literal, ($fn_one: path,$text_one: literal)) => {
        Solution {
            input: $file,
            one: to_solver!($text_one, $fn_one),
            two: None,
        }
    };
}

/// user controlled operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Quit the program
    Quit,
    /// Run solutions for all days
    All,
    /// Run solutions for a single day
    Day(usize),
    /// Run solutions for a range of days
    Range(RangeInclusive<usize>),
}

// parse a command from user input
impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let dash = spaces().and_then(p_char('-')).and_then(spaces());
        let range = keep_first(p_int(10), dash.clone())
            .and_then(p_int(10))
            .map(|(a, b)| Command::Range(a as usize..=b as usize));
        let day = p_int(10).map(|d| Command::Day(d as usize));
        let quit = any_of(['q', 'Q']).and_then(spaces()).map(|_| Command::Quit);
        let all = any_of(['a', 'A']).and_then(spaces()).map(|_| Command::All);

        let p_command = choice([range, day, all, quit]);

        match p_command.parse(input) {
            Ok((_, command)) => Ok(command),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_parser() {
        let msg = "should parse user input into a command";
        let expected = Command::Quit;
        let actual: Command = "q".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Command::Quit;
        let actual: Command = "Q".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Command::All;
        let actual: Command = "a".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Command::All;
        let actual: Command = "A".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Command::Range(10..=25);
        let actual: Command = "10- 25".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Command::Day(10);
        let actual: Command = "10".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let actual = "foo".parse::<Command>().is_err();
        assert!(actual, "{}", msg);
    }
}
