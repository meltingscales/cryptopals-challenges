use substring::Substring;

#[test]
fn test_base64encode() {
    assert_eq!("Rg==".to_string(), to_base64("F".to_string()));
    assert_eq!("RkZG".to_string(), to_base64("FFF".to_string()));

    assert_eq!(
        "TWFyeSBoYWQgYQ==".to_string(),
        to_base64("Mary had a".to_string())
    );

    assert_eq!(
        "TWFyeSBoYWQ=".to_string(),
        to_base64("Mary had".to_string())
    );
}

pub fn to_base64(s: String) -> String {
    const BASE64CHARS: &[u8] =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    const CHUNKSIZE: usize = 3;

    let mut data = s.as_bytes().to_vec();
    let mut padding_bytes: Vec<u8> = vec![];
    let mut vec_base64: Vec<u8> = vec![];
    let mut padding_num = data.len() % CHUNKSIZE;

    // add a right zero pad to make the data a multiple of 3 characters
    if padding_num > 0 {
        while padding_num < CHUNKSIZE {
            padding_bytes.push('=' as u8);
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
        let mut chunked_bytes: Vec<u8> = vec![];
        for i in 0..CHUNKSIZE {
            chunked_bytes.push(data[chunk_pos + i]);
        }

        // step over 3 numbers at a time
        let mut bytes_shifted: Vec<u32> = vec![];
        let mut shift_by = 16;
        for i in 0..CHUNKSIZE {
            bytes_shifted.push((chunked_bytes[i] as u32) << shift_by);
            shift_by -= 8;
            // println!("d{}:  {}", i, to_bitstring(darr[i], Some(24), None));
        }

        // 3 8bit numbers become one 24bit number
        let mut n = 0;
        for i in 0..CHUNKSIZE {
            n += bytes_shifted[i];
        }
        // println!("all: {}", to_bitstring(n, Some(24), None));

        let mut ascii_indices: Vec<u32> = vec![];

        shift_by = 18;
        for i in 0..4 {
            ascii_indices.push(((n >> shift_by) & 63));
            shift_by -= 6;
            // println!(
            //     "o{}:  {} or {}",
            //     i,
            //     to_bitstring(oarr[i], Some(6), Some(6)),
            //     oarr[i]
            // );

            vec_base64.push(BASE64CHARS[ascii_indices[i] as usize]);
        }
        // println!();
    }

    let mut return_val = String::from_utf8(vec_base64).unwrap();
    let padding = String::from_utf8(padding_bytes).unwrap();

    // remove zero pad
    return_val = return_val
        .substring(0, return_val.len() - padding.len())
        .to_string();

    // add the padding string

    return_val.push_str(padding.as_str());

    return return_val;
}
