// Kevin Cantu
// Boyer-Moore string searching

use std;

// Function: findn_str_between
//
// Boyer-Moore string search, which returns
// up to `nn` byte positions of matched substrings
fn findn_str_between (haystack: str, needle: str,
                      nn: uint,
                      start: uint, end: uint) -> [uint] {
   let results = [];

   let nlen = str::len(needle);
   let hlen = str::len(haystack);

   // empty haystack
   if hlen == 0u {
      ret results;
   }

   // empty needle
   if nlen == 0u {
      vec::push(results, 0u);
      ret results;
   }

   // needle too large
   if hlen < nlen {
      ret results;
   }

   // generate the tables
   let ct = char_table(needle);
   let pt = prefix_table(needle);

   // query both
   // based on position within the needle and character in haystack
   let getShift = fn@(pos: uint, ch: u8) -> uint {
      let charShift = ct[ch as uint];
      let prefShift = pt[nlen - 1u - pos];

      if charShift > prefShift {
         ret charShift;
      } else {
         ret prefShift;
      }
   };

   // step up through the haystack
   let outerii = start;
   while outerii < end - nlen + 1u {

      // step back through needle
      let windowii = nlen;
      while 0u < windowii
            && (outerii + windowii - 1u) < hlen  // don't exceed haystack
      {
         windowii -= 1u;

         if needle[windowii] == haystack[outerii+windowii] {
            // still matching

            if windowii == 0u {
               // matched
               vec::push(results, outerii);

               if vec::len(results) >= nn {
                  ret results;
               } else {
                  outerii += nlen;
               }
            }
         } else {
            // not matching yet or was partial match
            outerii += getShift(windowii, haystack[outerii+windowii]);
            break;
         }
      }
   }

   ret results;
}

// compute the table used to choose a shift based on
// an unmatched character's possible position within the search string
// (a.k.a. the bad-character table)
fn char_table (needle: str) -> [uint] {
   let len = str::len(needle);
   let mm  = vec::init_elt_mut(255u, len);

   let jj = len - 1u; // drop the last byte

   //assert 0u <= jj;
   //assert       jj < str::len(needle);

   // from last-1 to first
   while jj > 0u {
      jj -= 1u;

      let key = needle[jj] as uint;

      // if we haven't set it yet, set it now
      // (besides default)
      if mm[key] == len {
         mm[key] = len - 1u - jj;
      }
   }

   ret vec::from_mut(mm);
}

// compute the table used to choose a shift based on
// a partially matched suffix of the search string
// (a.k.a. the good-suffix table)
fn prefix_table (needle_str: str) -> [uint] {
   let needle = str::bytes(needle_str);

   let len   = vec::len(needle);
   //assert 0u < len;

   // initialize len chars to len
   let mm  = vec::init_elt_mut(len, len);

   // step to larger suffixes
   let sii = 0u;
   while sii < len {

      // tail of the needle we seek
      let suffix      = vec::slice(needle, len - sii,      len);
      let suffix_plus = vec::slice(needle, len - sii - 1u, len);
      let slen = vec::len(suffix);

      // step to smaller prefixes
      let pii = len - 1u;
      while pii > 0u {

         // a prefix of the needle
         let prefix = vec::slice(needle, 0u, pii);
         let plen = vec::len(prefix);

         // if suffix fully matched, or
         // prefix is bigger than suffix: only tail matched
         // (which we might jump to)
         if
            (plen <= slen
             && vec::ends_with(suffix, prefix))
         ||
            (slen < plen
             && vec::ends_with(prefix, suffix)
             && !vec::ends_with(prefix, suffix_plus))
         {
            // if we haven't set it yet, set it now
            // (besides default)
            if mm[sii] == len {
               mm[sii] = len-pii;
            }
         }

         pii -= 1u;
      }

      // if it hasn't been set, there was no matching prefix,
      // so set it now
      if mm[sii] == len {
         mm[sii] = len-pii;
      }

      sii += 1u;
   }

   ret vec::from_mut(mm);
}

fn findn_str(haystack: str, needle: str, nn: uint) -> [uint] {
   findn_str_between(haystack, needle, nn, 0u, str::len(haystack))
}

fn find_str(haystack: str, needle: str) -> option<uint> {
   let found = findn_str(haystack, needle, 1u);
   
   alt vec::len(found) {
      0u { option::none }
      xx { option::some(found[0u]) }
   }
}

#[test]
fn test_char_table () {
   let ct = char_table("ANPANMAN");

   assert 1u == ct['A' as uint];
   assert 2u == ct['M' as uint];
   assert 3u == ct['N' as uint];
   assert 5u == ct['P' as uint];

   // others
   assert 8u == ct['z' as uint];
   assert 8u == ct['w' as uint];
   assert 8u == ct['x' as uint];
}

#[test]
fn test_char_table_utf8() {
   let ct = char_table("ะเ"); //e0b8b0 e0b980

   assert 2u == ct[0x_e0_u];
   assert 4u == ct[0x_b8_u];
   assert 3u == ct[0x_b0_u];
   assert 2u == ct[0x_e0_u];
   assert 1u == ct[0x_b9_u];
   assert 6u == ct[0x_80_u];
}

#[test]
fn test_prefix_table_ascii() {
   let pt = prefix_table("ANPANMAN");

   assert 1u == pt[0u]; //        (n)
   assert 8u == pt[1u]; //       (a)n
   assert 3u == pt[2u]; //      (m)an
   assert 6u == pt[3u]; //     (n)man
   assert 6u == pt[4u]; //    (a)nman
   assert 6u == pt[5u]; //   (p)anman
   assert 6u == pt[6u]; //  (n)panman
   assert 6u == pt[7u]; // (a)npanman
}

#[test]
fn test_prefix_table_utf8() {
   let pt = prefix_table("ประเ");

   assert  1u == pt[0u];
   assert 12u == pt[3u];
   assert 12u == pt[6u];
   assert 12u == pt[9u];
}

#[test]
fn test_findn_str() {
   assert []       == findn_str("banana", "apple pie", 1u);
   assert [0u]     == findn_str("abcxxxxxx", "abc", 1u);
   assert [3u]     == findn_str("xxxabcxxx", "abc", 1u);
   assert [6u]     == findn_str("xxxxxxabc", "abc", 1u);
   assert [3u]     == findn_str("xxxabcabc", "abc", 1u);
   assert [3u, 6u] == findn_str("xxxabcabc", "abc", 5u);
   assert [3u, 7u] == findn_str("xxxabcxabc", "abc", 5u);
   assert [3u, 8u] == findn_str("xxxabcxxabc", "abc", 5u);
}

#[test]
fn test_find_strX_ascii() {
   assert option::none     == find_str("", "");
   assert option::none     == find_str("banana", "apple pie");
   assert option::some(0u) == find_str("abcxxxxxx", "abc");
   assert option::some(3u) == find_str("xxxabcxxx", "abc");
   assert option::some(6u) == find_str("xxxxxxabc", "abc");
}

#[test]
fn test_find_strX_utf8() {
   let data = "ประเทศไทย中华Việt Nam";

   assert option::some( 0u) == find_str(data, "");
   assert option::none      == find_str(data, "ไท华");
   assert option::some( 0u) == find_str(data, "ประเ");
   assert option::some( 3u) == find_str(data, "ระ");
   assert option::some( 6u) == find_str(data, "ะเ");
   assert option::some(15u) == find_str(data, "ศไทย中华");
   assert option::some(18u) == find_str(data, "ไทย中华");
   assert option::some(24u) == find_str(data, "ย中华");
   assert option::some(27u) == find_str(data, "中华");
}


