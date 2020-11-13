use crate::base64::to_base64;
use crate::bitstring::{to_bitstring, to_bitstring_4bytes, to_bitstring_straight};
use crate::util::{printHeader, read_datafile};
use substring::Substring;

pub fn runTest() {
    printHeader("Set 01, Challenge 01".to_string());

    let input_filepath = "../set-01/challenge-01/input";
    let output_filepath = "../set-01/challenge-01/output";

    let input: String = read_datafile(input_filepath.to_string());

    let expectedOutput = read_datafile(output_filepath.to_string());

    println!(
        "Does base64('{}') == '{}'?\n {}.",
        &input,
        to_base64(input.clone()),
        to_base64(input.clone()) == expectedOutput
    );

    assert_eq!(expectedOutput, to_base64(input));
}
