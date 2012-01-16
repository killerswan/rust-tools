
use std;

/*
Function: all

Return true if a predicate matches all characters

If the string contains no characters
*/
fn all<T>(v: [T], f: block(T) -> bool) -> bool {
    for elem: T in v { if !f(elem) { ret false; } }
    ret true;
}


/*
Function: reserve

Reserve capacity for `n` elements in a string
*/

//fn reserve(+ss: str, nn: uint) unsafe {
fn reserve_empty(nn: uint) -> str unsafe {
   // start with something
   let ss = "";

   // make a vector
   let vv: [u8] = unsafe::reinterpret_cast(ss);
   unsafe::leak(ss);
   
   // expand it to size + \0
   vec::reserve(vv, nn+1u);

   // make a string
   let ss_ : str = unsafe::reinterpret_cast(vv);
   unsafe::leak(vv);

   // return it
   ret ss_;
}



/*
Function: map

Apply a function to each character
*/
fn map(ss: str, ff: fn&(char) -> char) -> str {
    // FIXME: now lets benchmark this and show it is faster
    let result = reserve_empty(str::byte_len(ss));

    str::iter_chars(ss, {|cc|
        str::push_char(result, ff(cc));
    });

    ret result;
}

#[test]
fn test_map () {
    assert "" == map("", char::to_upper);
    assert "YMCA" == map("ymca", char::to_upper);
}


