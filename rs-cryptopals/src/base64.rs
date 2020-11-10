use substring::Substring;

pub fn to_base64(s: String) -> String {
    let BASE64CHARS = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let CHUNKSIZE = 3;

    let mut data = s.as_bytes().to_vec();
    let mut padding: Vec<u8> = vec![];
    let mut vec_base64: Vec<u8> = vec![];
    let mut padding_num = data.len() % CHUNKSIZE;

    // add a right zero pad to make the data a multiple of 3 characters
    if padding_num > 0 {
        while padding_num < CHUNKSIZE {
            padding.push('=' as u8);
            data.push('\x00' as u8); // This line specifically ensures we never have to check to see if our input string's length isn't a multiple of 3...
            padding_num += 1;
        }
    }

    // println!(
    //     "padding is '{}' for '{}'",
    //     String::from_utf8(padding.clone()).unwrap(),
    //     s
    // );

    // loop over 3 bytes at a time
    for chunk_pos in (0..data.len()).step_by(3) {
        let mut iarr: Vec<u8> = vec![];
        for i in 0..CHUNKSIZE {
            iarr.push(data[chunk_pos + i]);
        }

        // step over 3 numbers at a time
        let mut darr: Vec<u32> = vec![];
        let mut dshift = 16;
        for i in 0..CHUNKSIZE {
            darr.push((iarr[i] as u32) << dshift);
            dshift -= 8;
            // println!("d{}:  {}", i, to_bitstring(darr[i], Some(24), None));
        }

        // 3 8bit numbers become one 24bit number
        let mut n = 0;
        for i in 0..CHUNKSIZE {
            n += darr[i];
        }
        // println!("all: {}", to_bitstring(n, Some(24), None));

        let mut oarr: Vec<u32> = vec![];

        let mut oshift = 18;
        for i in 0..4 {
            oarr.push(((n >> oshift) & 63));
            oshift -= 6;
            // println!(
            //     "o{}:  {} or {}",
            //     i,
            //     to_bitstring(oarr[i], Some(6), Some(6)),
            //     oarr[i]
            // );

            vec_base64.push(BASE64CHARS[oarr[i] as usize]);
        }
        // println!();
    }

    let mut r = String::from_utf8(vec_base64).unwrap();
    let p = String::from_utf8(padding).unwrap();

    // remove zero pad
    r = r.substring(0, r.len() - p.len()).to_string();

    // add the padding string

    r.push_str(p.as_str());

    return r;
}
