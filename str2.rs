// Kevin Cantu
// additional string functions I've wanted in Rust

/* so far including:
   splitfn
   lines
   words
*/

#[link(name = "str2", author = "kcantu", vers = "0.0")];
use std;

// a better slice, unicode safe
// since strings are null terminated, cannot copy in place
//
// start is the character position to begin copying
// end is the character position which should not be copied
//
// Does not make internal conversion to char (which is UTF-32/UCS-4),
// but is UTF-8 aware.
//

// helper: note, slice_chars could be faster (by not starting byte_position from zero the second time)
// if this was a named block internal to slice_chars
fn byte_position (&&ss: str, char_position: uint) -> uint
{
   let cursor_char = 0u;
   let cursor_byte = 0u;

   while cursor_char < char_position {
      let sz = str::utf8_char_width(ss[cursor_byte]);
      assert sz > 0u;
      cursor_byte += sz;
      cursor_char += 1u;
   }
   assert cursor_char <= str::char_len(ss);
   ret cursor_char;
}

fn slice_chars(ss: str, begin: uint, end: uint) -> str unsafe
{
   assert begin <= end;
   assert end <= str::char_len(ss);

   // find the begin and end
   let byte_begin = byte_position(ss, begin);
   let byte_end   = byte_position(ss, end);

   // copy the bytes
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

#[test]
fn test_slice() {
    assert (str::eq("ab", slice_chars("abc", 0u, 2u)));
    assert (str::eq("bc", slice_chars("abc", 1u, 3u)));
    assert (str::eq("", slice_chars("abc", 1u, 1u)));
    fn a_million_letter_a() -> str {
        let i = 0;
        let rs = "";
        while i < 100000 { rs += "aaaaaaaaaa"; i += 1; }
        ret rs;
    }
    fn half_a_million_letter_a() -> str {
        let i = 0;
        let rs = "";
        while i < 100000 { rs += "aaaaa"; i += 1; }
        ret rs;
    }
    assert (str::eq(half_a_million_letter_a(),
                    slice_chars(a_million_letter_a(), 0u, 500000u)));
}

#[test]
fn test_slice_with_unicode() {
    fn a_million_letter_华() -> str {
        let i = 0;
        let rs = "";
        while i < 100000 { rs += "华华华华华华华华华华"; i += 1; }
        ret rs;
    }
    fn half_a_million_letter_华() -> str {
        let i = 0;
        let rs = "";
        while i < 100000 { rs += "华华华华华"; i += 1; }
        ret rs;
    }
    assert (str::eq(half_a_million_letter_华(),
                    slice_chars(a_million_letter_华(), 0u, 500000u)));
}

#[test]
fn equals_unicode() {
   let data = "ประเทศไทย中华Việt Nam";
   let data2 = "ประเทศไทย中华Việt Nam";
   assert str::eq(data,data2);

   assert str::eq("华", "华");
   assert str::eq("华华", "华华");
}


// a more general split, unicode safe
// using a function, e.g., char::is_whitespace instead of single u8
fn splitfn(ss: str, sepf: fn&(cc: char)->bool) -> [str]
{
   let vv: [str] = [];
   let accum: str = "";
   let ends_with_sep: bool = false;

   str::iter_chars(ss, {|cc|
      if sepf(cc) {
         vv += [accum];
         accum = "";
         ends_with_sep = true;
      } else {
         str::push_char(accum, cc);
         ends_with_sep = false;
      }
   });
   if str::char_len(accum) != 0u || ends_with_sep {
      vv += [accum];
   }
   ret vv;
}

#[test]
fn test_splitfn_a ()
{
   let data = "ประเทศไทย中华Việt Nam";
   assert ["ประเทศไทย中", 
           "Việt Nam"]
      == splitfn (data, {|cc| cc == '华'});
}

fn lines (ss: str) -> [str]
{
   ret splitfn(ss, {|cc| cc == '\n'});
}

#[test]
fn test_lines_splitfn ()
{
   let data = "\nMary had a little lamb\nLittle lamb\nLittle lamb\n";

   assert ["", "Mary had a little lamb", "Little lamb", "Little lamb", ""]
      == lines(data);
}

fn words (ss: str) -> [str]
{
   ret vec::filter( splitfn(ss, {|cc| char::is_whitespace(cc)}), 
                    {|w| 0u < str::char_len(w)});
}

#[test]
fn test_words_splitfn ()
{
   let data = "\nMary had a little lamb\nLittle lamb\nLittle lamb\n";

   assert ["Mary","had","a","little","lamb","Little","lamb","Little","lamb"]
      == words(data);
}

#[test]
fn test_example ()
{
   let s = "中华Việt Nam";
   let i = 0u;
   while i < str::byte_len(s)
   {
      let {ch:c, next:j} = str::char_range_at(s, i);
      std::io::println(#fmt("%u: %c",i,c));
      i = j;
   }

   /* Output: 
      0: 中
      3: 华
      6: V
      7: i
      8: ệ
      11: t
      12:  
      13: N
      14: a
      15: m
   */
}


