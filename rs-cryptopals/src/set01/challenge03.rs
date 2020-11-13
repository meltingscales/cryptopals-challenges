use crate::util::{printHeader, read_datafile, read_datafile_bytes, read_datafile_n_bytes};
use std::collections::HashMap;
use std::string::FromUtf8Error;

pub fn runTest() {
    printHeader("Set 01, Challenge 03".to_string());

    let bible_input: Vec<u8> = read_datafile_n_bytes(
        "../set-01/challenge-03/The Project Gutenberg EBook of The King James Bible.txt"
            .to_string(),
        100_000,
    );

    let mut char_freq: HashMap<u8, u128> = HashMap::new();

    // go over every char in input and add up how many times they show up
    for i in 0..bible_input.len() {
        let byte: u8 = *bible_input.get(i).unwrap();
        if char_freq.contains_key(&byte) {
            // increment 1, we've seen this key already
            char_freq.insert(byte, char_freq.get(&byte).unwrap() + (1));
        } else {
            // we've never seen this key before, put '1'
            char_freq.insert(byte, 1);
        }
    }

    for (byte, occurrences) in char_freq.into_iter() {
        // if we get UTF8Error, return "???"
        let mut char = match String::from_utf8(vec![byte]) {
            Err(_) => "???".to_string(),
            Ok(c) => c,
        };

        if char == "\n" || char == "\r" {
            char = "\\n".to_string()
        }

        println!("{:3} aka `{}` occurs {} times.", byte, char, occurrences);
    }
}
