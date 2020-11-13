use std::u32::MAX;

#[test]
pub fn testBitstringFn() {
    // Test bit shifting...
    // println!(
    //     "{} >> 2 = {}",
    //     to_bitstring_straight(0b11101, Some(5)),
    //     to_bitstring_straight(0b11101 >> 2, Some(5))
    // );

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

pub fn to_bitstring_4bytes(num: u32, _spacing: Option<u32>) -> String {
    return to_bitstring(num, Some(4 * 8), _spacing);
}

pub fn to_bitstring_byte(num: u32) -> String {
    return to_bitstring(num, Some(8), None);
}

pub fn to_bitstring_straight(num: u32, _width: Option<u32>) -> String {
    return to_bitstring(num, _width, Some(MAX));
}

pub fn to_bitstring(num: u32, _width: Option<u32>, _spacing: Option<u32>) -> String {
    let width = _width.unwrap_or(4 * 8);
    let spacing = _spacing.unwrap_or(8);

    let mut n = num;

    let mut s = "".to_string();
    for i in 0..width {
        let bit = n & 1;

        s = format!("{}{}", bit.to_string(), s);

        // time to put a space. We're appending backwards so...
        if (((width - i) - 1) % spacing) == 0 {
            if i != 0 && i != width - 1 {
                s = format!("{}{}", " ", s);
            }
        }

        n = n >> 1;
    }

    return s;
}
