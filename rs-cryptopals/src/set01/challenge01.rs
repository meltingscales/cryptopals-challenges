use crate::base64::to_base64;
use crate::bitstring::{to_bitstring, to_bitstring_4bytes, to_bitstring_straight};
use crate::util::read_datafile;
use substring::Substring;

pub fn runTest() {
    let input_filepath = "../set-01/challenge-01/input";
    let output_filepath = "../set-01/challenge-01/output";

    let input: String = read_datafile(input_filepath.to_string());

    let output = read_datafile(output_filepath.to_string());

    println!(
        "Does base64('{}') == '{}'?\n {}.",
        input.clone(),
        to_base64(input.clone()),
        to_base64(input.clone()) == output
    );

    assert_eq!(output, to_base64(input));
}
