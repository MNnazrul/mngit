pub fn pad_sha1(input: &[u8]) -> Vec<u8> {
    let bit_len: u64 = (input.len() as u64) * 8;

    let mut msg = Vec::with_capacity(input.len() + 1 + 8 + 64);
    msg.extend_from_slice(input);

    msg.push(0x80);

    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }

    msg.extend_from_slice(&bit_len.to_be_bytes());

    debug_assert_eq!(msg.len() % 64, 0);

    msg
}
