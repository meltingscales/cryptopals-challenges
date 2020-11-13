use std::fs::File;
use std::io::Read;

pub fn printHeader(s: String) {
    println!(" *** {} ***", s);
}

/// Read all content from a file into a `Vec<u8>` as bytes.
pub fn read_datafile_bytes(filepath: String) -> Vec<u8> {
    return read_datafile(filepath).into_bytes();
}

/// Read up to `n` bytes starting from 0 from `filepath`.
pub fn read_datafile_n_bytes(filepath: String, n: usize) -> Vec<u8> {
    let mut f: File = match File::open(&filepath) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file '{:?}': {:?}", filepath, error),
    };

    let file_size = f.metadata().unwrap().len();
    if file_size < (n as u64) {
        panic!(format!(
            "File's size is {} which is less than {}!",
            file_size, n
        ));
    }

    let mut buffer = vec![0; n];

    f.read(&mut buffer).unwrap();

    return buffer;
}

pub fn read_datafile(filepath: String) -> String {
    let mut f: File = match File::open(&filepath) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file '{:?}': {:?}", filepath, error),
    };

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    return contents;
}
