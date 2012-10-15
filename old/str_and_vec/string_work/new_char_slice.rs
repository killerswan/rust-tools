fn char_slice(ss: str, begin: uint, end: uint) -> str unsafe {
    // resist the temptation to check that begin and end are reasonable

    // find the begin and end
    let byte_begin = char_to_byte_offset(ss, begin);
    let byte_end   = char_to_byte_offset(ss, end);

    // copy the bytes
    // we have to copy so we can null terminate
    let new_vector = [];
    let ii = byte_begin;

    while ii < byte_end {
        vec::push(new_vector, ss[ii]);
        ii += 1u;
    }
    vec::push(new_vector, 0u8);

    // return as a string
    let new_string: str = unsafe::reinterpret_cast(new_vector);
    unsafe::leak(new_vector);
    ret new_string;
}

fn char_to_byte_offset (&&ss: str, char_offset: uint) -> uint {
    // resist the temptation to check that the char_offset is reasonable

    let cursor_char = 0u;
    let cursor_byte = 0u;

    while cursor_char < char_offset {
        let sz = str::utf8_char_width(ss[cursor_byte]);
        assert sz > 0u;
        cursor_byte += sz;
        cursor_char += 1u;
    }

    assert cursor_char <= str::char_len(ss);
    assert cursor_byte <= str::byte_len(ss);
    ret cursor_byte;
}


