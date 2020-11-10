use std::u32::MAX;

pub fn to_bitstring_4bytes(num: u32, _spacing: Option<u32>) -> String {
    return to_bitstring(num, Some(3 * 8), _spacing);
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
