// #![deny(clippy::all)]
// #![warn(clippy::pedantic)]
// #![allow(clippy::single_match)]

use log::trace;

use std::env;
use std::fs;

use svg2polylines::*;

pub fn svg_to_ascii(svg_file_path: &str) {
    if let contents = fs::read_to_string(svg_file_path) {
        println!("{:?}", contents);

        if let res = parse(&contents.unwrap()) {
            for pair in res.unwrap() {
                println!("starting new pair");

                for i in 0..pair.len() {
                    if i < (pair.len() - 1) {
                        let current_pair: CoordinatePair = *pair.get(i).unwrap();
                        let next_pair: CoordinatePair = *pair.get(i + 1).unwrap();

                        // iterate...

                        println!("COORD: {:?}", current_pair.x);
                    }
                }
            }
        }
    }
}
