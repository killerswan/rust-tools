
use std;


fn char_to_byte_offset (&&ss: str, char_position: uint) -> uint {
    let cursor_char = 0u;
    let cursor_byte = 0u;

    while cursor_char < char_position {
        let sz = str::utf8_char_width(ss[cursor_byte]);
        assert sz > 0u;
        cursor_byte += sz;
        cursor_char += 1u;
    }
    assert cursor_char <= str::char_len(ss);
    assert cursor_byte <= str::byte_len(ss);
    ret cursor_byte;
}


#[test]
fn test () {
    let s = "ประเทศไทย中华Việt Nam";
    let i = 0u;
    while i < str::byte_len(s) {
       let {ch, next} = str::char_range_at(s, i);
       std::io::println(#fmt("%u: %c",i,ch));
       i = next;
    }
}

