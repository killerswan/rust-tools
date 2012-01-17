use std;

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

fn report_time <XX> (desc: str, ff: fn&() -> XX) {
   let t0 = std::time::precise_time_s();
   ff();
   let t1 = std::time::precise_time_s();

   std::io::println(desc + " " + #fmt("%06.3f sec", t1 - t0));
}

fn main () {
   fn word_of_god () -> str {
      std::io::println ("Loading the lolcat bible...");
      let path = "../lolcat/LOLCatBible_2012-01-04.xml";
      let bible = std::io::read_whole_file (path);
      let bible_ = str::unsafe_from_bytes (result::get (bible));
      //ret str::char_slice(bible_, 0u, 20000u);
      ret bible_;
   }

   let meow = word_of_god();

   let _x = map_slow(meow, char::to_upper);
   let _x = map(meow, char::to_upper);
}


