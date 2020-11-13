use std::fs::File;
use std::io::Read;

pub fn printHeader(s: String) {
    println!(" *** {} ***", s);
}

pub fn read_datafile(filepath: String) -> String {
    let mut f: File = match File::open(filepath.clone()) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file '{:?}': {:?}", filepath, error),
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    return contents;
}
