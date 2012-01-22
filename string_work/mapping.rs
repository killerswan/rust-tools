
use std;
use meow;

/*
Function: all

Return true if a predicate matches all characters

If the string contains no characters
*/
fn all(ss: str, ff: fn&(char) -> bool) -> bool {
    str::loop_chars(ss, ff)
}

/*
Function: reserve

Reserve capacity for `n` elements in a string
*/
fn reserve(&ss: str, nn: uint) unsafe {
   // start with an existing string

   str::char_len(ss); std::io::println("+++A+++");

   // make a vector
   let vv: [u8] = unsafe::reinterpret_cast(ss);
   
   str::char_len(ss); std::io::println("+++B+++");

   // expand it to size + \0
   vec::reserve(vv, nn+1u);

   // FIXME:
   // fails sometimes on assertion in byte_len called by char_len
   str::char_len(ss); std::io::println("+++C+++");

   // forget the vector
   unsafe::leak(vv);

   str::char_len(ss); std::io::println("+++D+++");
}

/*
Function: reserve_empty

Create and reserve capacity for `n` elements in a string
*/
fn reserve_empty(nn: uint) -> str unsafe {
   let ss = "";
   reserve(ss, nn);
   ret ss;
}

/*
Function: map

Apply a function to each character
*/
fn map_slow(ss: str, ff: fn&(char) -> char) -> str {
    let result = "";

    str::iter_chars(ss, {|cc|
        str::push_char(result, ff(cc));
    });

    ret result;
}

fn mapX(ss: str, ff: fn&(char) -> char) -> str {
    let result = reserve_empty(str::byte_len(ss));

    str::iter_chars(ss, {|cc|
        str::push_char(result, ff(cc));
    });

    ret result;
}
fn map(ss: str, ff: fn&(char) -> char) -> str {
   let result = "";
   reserve(result, str::byte_len(ss));

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

#[test]
fn test_map_slow () {
    assert "" == map_slow("", char::to_upper);
    assert "YMCA" == map_slow("ymca", char::to_upper);
}

#[test]
fn test_mapX () {
    assert "" == mapX("", char::to_upper);
    assert "YMCA" == mapX("ymca", char::to_upper);
}

#[test]
fn test_all () {
    assert true  == all("", char::is_uppercase);
    assert false == all("ymca", char::is_uppercase);
    assert true  == all("YMCA", char::is_uppercase);
    assert false == all("yMCA", char::is_uppercase);
    assert false == all("YMCy", char::is_uppercase);
}

fn main () {
   let f = bind map_slow(_,char::to_upper);

   fn f_ () { map_slow(meow::sample_string(), char::to_upper); }
   meow::compare("compare map_slow vs. map_slow", f_, f_);
   meow::compare_several("compare several map_slow vs. map_slow", f_, f_);

   //meow::compare_sweep_strings("compare sweep strings", f, f);
   meow::compare_sweep_strings_lim("compare sweep strings", f, f, 18u);

}


