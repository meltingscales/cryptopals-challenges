use crate::base64::to_base64;
use crate::bitstring::{to_bitstring, to_bitstring_4bytes, to_bitstring_straight};
use crate::util::read_datafile;
use substring::Substring;

pub fn testBitstringFn() {
    // Test bit shifting...
    println!(
        "{} >> 2 = {}",
        to_bitstring_straight(0b11101, Some(5)),
        to_bitstring_straight(0b11101 >> 2, Some(5))
    );

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
}

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

    assert_eq!(1, 1);
    assert_eq!("lol", "lol");
    assert_eq!("lol".to_string().substring(1, 2), "o".to_string());
    assert_eq!(
        "TWFyeSBoYWQgYQ==".to_string(),
        to_base64("Mary had a".to_string())
    );
    assert_eq!(output, to_base64(input));
}
