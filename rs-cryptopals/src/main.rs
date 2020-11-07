#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

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
    let mut data = s.as_bytes().to_vec();
    let mut padding: Vec<u8> = vec![];
    let vec_base64: Vec<u8> = vec![];
    let mut padding_num = data.len() % 3;

    // add a right zero pad to make the data a multiple of 3 characters
    if padding_num > 0 {
        while padding_num < 3 {
            padding.push('=' as u8);
            data.push(0 as u8);
            padding_num += 1;
        }
    }

    // loop over 3 bytes at a time
    for i in (0..data.len()).step_by(3) {
        //TODO lol
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
    r = r.substring(0, (r.len()-p.len())).to_string();

    // add the padding string

    r.push_str(p.as_str());

    return r;
}

fn main() {
    let input_filepath = "../set-01/challenge-01/input";
    let output_filepath = "../set-01/challenge-01/output";
    let table_filepath = "../set-01/challenge-01/table";


    println!("Hello, world!");

    let input: String = read_datafile(input_filepath.to_string());
    println!("{}", input);

    let output = read_datafile(output_filepath.to_string());

    assert_eq!(1, 1);
    assert_eq!("lol", "lol");
    assert_eq!("lol".to_string().substring(1,2), "o".to_string());
    assert_eq!("eA==".to_string(), to_base64("x".to_string()));
    assert_eq!(output, to_base64(input));
}
