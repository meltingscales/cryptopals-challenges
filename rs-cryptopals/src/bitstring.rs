pub fn to_bitstring(num: u32, width: u32) -> String
{
    let mut n = num;

    let mut s = "".to_string();
    for _ in 0..width {
        let bit = n & 1;

        s = format!("{}{}", bit.to_string(), s);

        n = n >> 1;
    }

    return s;
}
