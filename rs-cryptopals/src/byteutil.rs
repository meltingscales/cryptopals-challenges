use hex::FromHexError;

#[test]
fn test_hexstring_to_bytes() {
    assert_eq!(vec![0xff, 0x01], decode_hex("ff01".to_string()).unwrap())
}

#[test]
fn test_xor_vec_u8() {
    assert_eq!(
        xor_vec_u8(vec![0xff, 0xff, 0xff], vec![0x00, 0x00, 0x00]),
        vec![0xff, 0xff, 0xff]
    );
    assert_eq!(
        xor_vec_u8(vec![0b01010101, 0xff, 0xff], vec![0b10101010, 0x00, 0x00]),
        vec![0xff, 0xff, 0xff]
    );
}

// TODO: Implement this yourself and don't use the method.
pub fn decode_hex(s: String) -> Result<Vec<u8>, FromHexError> {
    return hex::decode(s);
}

// TODO: Implement this yourself and don't use the method.
pub fn encode_hex(s: Vec<u8>) -> String {
    return hex::encode(s);
}

/// Take 2 `vec<u8>` and xor them.
pub fn xor_vec_u8(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    assert_eq!(vec1.len(), vec2.len());

    let mut return_vec: Vec<u8> = vec![];

    for i in 0..vec1.len() {
        return_vec.push(vec1[i] ^ vec2[i]);
    }

    return return_vec;
}
