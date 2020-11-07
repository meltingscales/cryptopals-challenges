use std::{fs::File, io::Read, path::Path};

fn read_datafile(filepath: String) -> String {
    let mut f: File = match File::open(filepath.clone()) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file '{:?}': {:?}", filepath, error),
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    return contents;
}

fn main() {
    let filepath = "../set-01/challenge-01/input";

    println!("Hello, world!");

    let data:String=read_datafile(filepath.to_string());
    print!("{}", data);
}
