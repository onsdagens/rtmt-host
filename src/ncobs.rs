/// N-COBS decoder
/// Takes a mutable reference to a (N-COBS encoded) vector of bytes. Returns a vector of "decoded" bytes, i.e.
/// with each zero pointer replaced by an actual zero.
/// This will "pop" the decoded frame from the input vector, leaving only the yet undecoded bytes.
pub fn decode(bytes: &mut Vec<u8>) -> Vec<u8> {
    let mut decoded_bytes = vec![];
    let mut eof = true;
    let mut last = false;
    let mut next_zero_offset = 0;
    // go from the back, thats where EOF is
    while let Some(byte) = bytes.pop() {
        // if the byte is a zero, this is the end of frame
        if byte == 0 {
            // EOF, skip
            continue;
        }
        // else, if we just encountered the end of frame, we are at a zero pointer.
        else if eof {
            eof = false;
            next_zero_offset = (byte as i8).abs() - 1;
            continue;
        }
        // if we are at an implied 0
        if next_zero_offset == 0 {
            // the value is a pointer to the next 0 (or start of frame)
            next_zero_offset = (byte as i8).abs();
            // if this is not the start of frame
            if !last {
                // replace the byte with a 0
                decoded_bytes.push(0u8);
                // if the byte is negative, the pointer points to start of frame
                if (byte as i8) < 0 {
                    next_zero_offset -= 1;
                    last = true;
                }
            } else {
                // if this is the start of frame, push the byte to decoded buffer and stop popping from encoded buffer
                decoded_bytes.push(byte);
                break;
            }
        } else {
            // if we are not at an implied zero, just push the byte.
            decoded_bytes.push(byte);
        }
        // decrease the pointer to the next zero or start of frame (since we are moving to the next value)
        next_zero_offset -= 1;
    }
    // since we went from the back, the decoded frame is in reverse.
    decoded_bytes.reverse();
    decoded_bytes
}
