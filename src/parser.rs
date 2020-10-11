#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::single_match)]
#![allow(clippy::cast_possible_truncation)]

use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::vec::Vec;

use svg2polylines::{parse, CoordinatePair};

static FILLER: char = '#';

#[derive(Debug)]
pub struct AsciiResult {
    data: Vec<Vec<char>>,
}

impl Display for AsciiResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buffer = if let Some(row) = self.data.get(0) {
            String::with_capacity(row.len() * self.data.len())
        } else {
            String::new()
        };

        for row in &self.data {
            for ch in row {
                buffer.push(*ch);
            }
            buffer.push('\n');
        }

        write!(f, "{}", buffer)
    }
}

pub fn svg_to_ascii(svg_file_path: &str) -> AsciiResult {
    let mut ascii_array = vec![vec![' '; 46]; 24];

    if let Ok(contents) = fs::read_to_string(svg_file_path) {
        if let Ok(res) = parse(&contents) {
            for pair in res {
                for i in 0..pair.len() {
                    if i < (pair.len() - 1) {
                        let current_pair: CoordinatePair = *pair.get(i).unwrap();
                        let next_pair: CoordinatePair = *pair.get(i + 1).unwrap();

                        let (from_x, to_x) = if current_pair.x < next_pair.x {
                            (current_pair.x.round() as u64, next_pair.x.round() as u64)
                        } else {
                            (next_pair.x.round() as u64, current_pair.x.round() as u64)
                        };

                        let (from_y, to_y) = if current_pair.y < next_pair.y {
                            (current_pair.y.round() as u64, next_pair.y.round() as u64)
                        } else {
                            (next_pair.y.round() as u64, current_pair.y.round() as u64)
                        };

                        for x in from_x..=to_x {
                            for y in from_y..=to_y {
                                if let Some(elem) = ascii_array.get_mut(y as usize) {
                                    if let Some(elem) = elem.get_mut(x as usize) {
                                        *elem = FILLER;
                                    }
                                }
                            }
                        }

                        let (from_x, to_x) = if current_pair.x < next_pair.x {
                            (current_pair.x as u64, next_pair.x as u64)
                        } else {
                            (next_pair.x as u64, current_pair.x as u64)
                        };

                        let (from_y, to_y) = if current_pair.y < next_pair.y {
                            (current_pair.y as u64, next_pair.y as u64)
                        } else {
                            (next_pair.y as u64, current_pair.y as u64)
                        };

                        for x in from_x..=to_x {
                            for y in from_y..=to_y {
                                if let Some(elem) = ascii_array.get_mut(y as usize) {
                                    if let Some(elem) = elem.get_mut(x as usize) {
                                        *elem = FILLER;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    AsciiResult { data: ascii_array }
}
