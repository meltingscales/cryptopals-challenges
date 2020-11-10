#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

// see https://en.wikibooks.org/wiki/Algorithm_Implementation/Miscellaneous/Base64#Javascript

mod bitstring;

use crate::bitstring::{
    to_bitstring, to_bitstring_4bytes, to_bitstring_byte, to_bitstring_straight,
};
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
    for chunk_pos in (0..data.len()).step_by(3) {
        let mut iarr: Vec<u8> = vec![];
        for i in 0..3 {
            iarr.push(data[chunk_pos + i]);
        }

        // step over 3 numbers at a time
        let mut darr: Vec<u32> = vec![];
        let mut dshift = 16;
        for i in 0..3 {
            darr.push((iarr[i] as u32) << dshift);
            dshift -= 8;
            println!("d{}:  {}", i, to_bitstring(darr[i], Some(24), None));
        }

        // 3 8bit numbers become one 24bit number
        let n: u32 = darr[0] + darr[1] + darr[2];
        println!("all: {}\n", to_bitstring(n, Some(24), None));

        // let mut o: Vec<u8> = vec![];
        //
        // o.append()
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
    assert_eq!(to_bitstring(0b001, Some(4), Some(4)), "0001");
    assert_eq!(to_bitstring(1, Some(2), Some(4)), "01");
    assert_eq!(
        to_bitstring(0xfffffffe as u32, Some(32), Some(32)),
        "11111111111111111111111111111110"
    );
    assert_eq!(
        to_bitstring_4bytes(0xfffffffe as u32, Some(8)),
        "11111111 11111111 11111111 11111110"
    );
    assert_eq!(
        to_bitstring(0x00fadfad as u32, Some(24), Some(6)),
        "111110 101101 111110 101101"
    );
    assert_eq!(
        to_bitstring(0xfffffffe as u32, Some(31), Some(32)),
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
