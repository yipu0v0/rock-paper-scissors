use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use std::str::FromStr;
use std::fmt;

use super::common::{GameError, MAX_PLAYER_COUNT};

#[derive(Clone, Copy, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Shape {
    pub fn get_result(shapes: &Vec<Shape>) -> Option<usize> {
        assert!(shapes.len() == MAX_PLAYER_COUNT as usize);
        if shapes[0] == shapes[1] {
            // draw
            return None;
        } else {
            if shapes[0] as u8 == (shapes[1] as u8 + 1) % 3 {
                return Some(0);
            } else {
                return Some(1);
            }
        }
    }
}

impl FromStr for Shape {
    type Err = GameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Rock"      => Ok(Shape::Rock),
            "Paper"     => Ok(Shape::Paper),
            "Scissors"  => Ok(Shape::Scissors),
            _           => Err(GameError::IllegalShape),
        }
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Shape::Rock        => "Rock",
            Shape::Paper       => "Paper",
            Shape::Scissors    => "Scissors"
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::Shape;
    #[test]
    fn result() {
        assert_eq!(Shape::get_result(&vec![Shape::Rock, Shape::Rock]), None);
        assert_eq!(Shape::get_result(&vec![Shape::Paper, Shape::Paper]), None);
        assert_eq!(Shape::get_result(&vec![Shape::Scissors, Shape::Scissors]), None);

        assert_eq!(Shape::get_result(&vec![Shape::Rock, Shape::Paper]), Some(1));
        assert_eq!(Shape::get_result(&vec![Shape::Paper, Shape::Rock]), Some(0));

        assert_eq!(Shape::get_result(&vec![Shape::Paper, Shape::Scissors]), Some(1));
        assert_eq!(Shape::get_result(&vec![Shape::Scissors, Shape::Paper]), Some(0));

        assert_eq!(Shape::get_result(&vec![Shape::Scissors, Shape::Rock]), Some(1));
        assert_eq!(Shape::get_result(&vec![Shape::Rock, Shape::Scissors]), Some(0));
    }
}
