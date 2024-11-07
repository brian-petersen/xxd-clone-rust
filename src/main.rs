use std::io::{self, BufReader, Read};

use itertools::Itertools;

const CHUNK_SIZE: usize = 16;
const GROUP_SIZE: usize = 2;

const LINE_WIDTH: usize = 8;

fn main() {
    let reader = BufReader::new(io::stdin());

    let mut line: usize = 0;

    for line_chunk in &reader.bytes().chunks(CHUNK_SIZE) {
        print!("{:0>width$x}: ", line, width = LINE_WIDTH);
        line += CHUNK_SIZE;

        for group_chunk in &line_chunk.map(|x| x.unwrap()).chunks(GROUP_SIZE) {
            for byte in group_chunk {
                // Each byte is printed as a two character hex
                print!("{:0>width$x}", byte, width = 2);
            }
            print!(" ");
        }
        println!();
    }
}
