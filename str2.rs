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
fn byte_position (&&ss: str, char_position: uint) -> uint
{
   let cursor_char = 0u;
   let cursor_byte = 0u;
   
   while cursor_char < char_position {
      cursor_byte += str::utf8_char_width(ss[cursor_byte]);
      cursor_char += 1u;
   }
   ret cursor_char;
}

fn slice_chars(ss: str, start: uint, end: uint) -> str unsafe
{
   // FIXME: should we make some assertions here?

   // find the start and end
   let byte_start = byte_position(ss, start);
   let byte_end = byte_position(ss, end);

   // copy the bytes
   let new_vector = [];
   let ii = byte_start;

   while ii < byte_end {
      vec::push(new_vector, ss[ii]);
   }
   vec::push(new_vector, 0u8);

   // return as a string
   let new_string: str = unsafe::reinterpret_cast(new_vector);
   unsafe::leak(new_vector);
   ret new_string;
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
   let s = "ประเทศไทย中华Việt Nam";
   let i = 0u;
   while i < str::byte_len(s)
   {
      let {ch:c, next:j} = str::char_range_at(s, i);
      std::io::println(#fmt("%u: %c",i,c));
      //std::io::println(#fmt("%u: %c (%c)",i,c,s[i] as char)); // note that s[i] is str[uint] -> u8
      i = j;
   }

   /* Output: 
      0: ป
      3: ร
      6: ะ
      9: เ
      12: ท
      15: ศ
      18: ไ
      21: ท
      24: ย
      27: 中
      30: 华
      33: V
      34: i
      35: ệ
      38: t
      39:  
      40: N
      41: a
      42: m
   */
}


