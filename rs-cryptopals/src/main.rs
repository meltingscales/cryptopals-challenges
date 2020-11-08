#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

// see https://en.wikibooks.org/wiki/Algorithm_Implementation/Miscellaneous/Base64#Javascript

mod bitstring;

use bitstring::to_bitstring;
use std::{fs::File, io::Read};
use substring::Substring;

fn read_datafile(filepath: String) -> String {
    let mut f: File = match File::open(filepath.clone()) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file '{:?}': {:?}", filepath, error),
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    return contents;
}

fn to_base64(s: String) -> String {
    let base64chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

    let mut data = s.as_bytes().to_vec();
    let mut padding: Vec<u8> = vec![];
    let vec_base64: Vec<u8> = vec![];
    let mut padding_num = data.len() % 3;

    // add a right zero pad to make the data a multiple of 3 characters
    if padding_num > 0 {
        while padding_num < 3 {
            padding.push('=' as u8);
            data.push('\x00' as u8); // This line specifically ensures we never have to check to see if our input string's length isn't a multiple of 3...
            padding_num += 1;
        }
    }

    println!(
        "padding is '{}' for '{}'",
        String::from_utf8(padding.clone()).unwrap(),
        s
    );

    // loop over 3 bytes at a time
    for i in (0..data.len()).step_by(3) {
        //TODO lol

        let i1 = data[i + 0];
        let i2 = data[i + 1];
        let i3 = data[i + 2];

        // step over 3 numbers at a time
        let d1 = (i1 as u32) << 16;
        let d2 = (i2 as u32) << 8;
        let d3 = (i3 as u32) << 0;

        println!("chunk: '{}'", String::from_utf8(vec![i1, i2, i3]).unwrap());
        println!("d1:  {}", to_bitstring(d1, 24));
        println!("d2:  {}", to_bitstring(d2, 24));
        println!("d3:  {}", to_bitstring(d3, 24));

        // 3 8bit numbers become one 24bit number
        let n: u32 = d1 + d2 + d3;
        println!("all: {}\n", to_bitstring(n, 24));

        // let
    }

    // println!("{}", data.len());

    //
    // for i in 0..data.len() {
    //     let b: u8 = data[i];
    //     // print!("{}", char::from(b));
    //     println!("at i={}, char={} and number is {}",i,char::from(b), b);
    // }

    // add padding as necessary
    // let padding = data.len() % 3;
    // for i in 0..padding{
    //     vec_base64.push('=' as u8);
    // }

    let mut r = String::from_utf8(vec_base64).unwrap();
    let p = String::from_utf8(padding).unwrap();

    // remove zero pad
    r = r.substring(0, r.len() - p.len()).to_string();

    // add the padding string

    r.push_str(p.as_str());

    return r;
}

fn main() {
    assert_eq!(to_bitstring(0b001, 4), "0001");
    assert_eq!(to_bitstring(1, 2), "01");
    assert_eq!(
        to_bitstring(0xfffffffe as u32, 32),
        "11111111111111111111111111111110"
    );
    assert_eq!(
        to_bitstring(0xfffffffe as u32, 31),
        "1111111111111111111111111111110"
    );

    let input_filepath = "../set-01/challenge-01/input";
    let output_filepath = "../set-01/challenge-01/output";
    let table_filepath = "../set-01/challenge-01/table";

    println!("Hello, world!");

    let input: String = read_datafile(input_filepath.to_string());
    println!("{}", input);

    let output = read_datafile(output_filepath.to_string());

    assert_eq!(1, 1);
    assert_eq!("lol", "lol");
    assert_eq!("lol".to_string().substring(1, 2), "o".to_string());
    assert_eq!(
        "TWFyeSBoYWQgYQ==".to_string(),
        to_base64("Mary had a".to_string())
    );
    assert_eq!(output, to_base64(input));
}
