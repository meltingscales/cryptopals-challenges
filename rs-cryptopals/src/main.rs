#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use std::{fs::File, io::Read};

fn read_datafile(filepath: String) -> String {
    let mut f: File = match File::open(filepath.clone()) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file '{:?}': {:?}", filepath, error),
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    return contents;
}

fn to_base64(data: Vec<u8>) -> Vec<u8> {
    return vec![1, 2, 3];
}

fn to_vec_u8(s: String) -> Vec<u8> {
    let ret_vec: Vec<u8> = vec![];

    let mut i = 0;
    for c in s.chars() {
        println!("i={}, Turning {} to a u8 object...",i, c);
        i+=1;
        //todo
    }

    return vec![1, 2, 3];
}

fn main() {
    let filepath = "../set-01/challenge-01/input";

    println!("Hello, world!");

    let data: String = read_datafile(filepath.to_string());
    println!("{}", data);

    println!("{:#?}", to_vec_u8(data))
}
