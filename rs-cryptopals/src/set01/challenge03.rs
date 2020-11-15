use crate::byteutil::byte_frequency;
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

    let char_freq: HashMap<u8, u32> = byte_frequency(&bible_input);

    let total_chars: usize = bible_input.len();
    println!("Total chars: {}", total_chars);
    for (byte, occurrences) in char_freq.into_iter() {
        // if we get UTF8Error, return "???"
        let mut char = match String::from_utf8(vec![byte]) {
            Err(_) => "???".to_string(),
            Ok(c) => c,
        };

        // handle newlines
        if char == "\n" {
            char = "\\n".to_string()
        } else if char == "\r" {
            char = "\\r".to_string()
        }

        println!(
            "{:3} or [{:^5}] occurs {:6} times/total = {3:.5}%",
            byte,
            char,
            occurrences,
            ((occurrences as f64) / (total_chars as f64)) * 100.0
        );
    }
}
