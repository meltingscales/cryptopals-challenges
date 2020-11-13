use crate::byteutil;
use crate::byteutil::{decode_hex, encode_hex, xor_vec_u8};
use crate::util::{printHeader, read_datafile};

pub fn runTest() {
    printHeader("Set 01, Challenge 02".to_string());

    let input1 = read_datafile("../set-01/challenge-02/input1".to_string());
    let input1Bytes = decode_hex(input1.clone()).unwrap();

    let input2 = read_datafile("../set-01/challenge-02/input2".to_string());
    let input2Bytes = decode_hex(input2.clone()).unwrap();

    let expectedOutput = read_datafile("../set-01/challenge-02/output".to_string());
    let expectedOutputBytes = decode_hex(expectedOutput.clone()).unwrap();

    let outputBytes = xor_vec_u8(input1Bytes, input2Bytes);
    let output = encode_hex(outputBytes.clone());

    println!(
        "Does\n{}\n XOR \n{} \n == \n{}\n ? {}",
        input1.clone(),
        input2.clone(),
        expectedOutput.clone(),
        output == expectedOutput
    );

    assert_eq!(outputBytes, expectedOutputBytes);
}
