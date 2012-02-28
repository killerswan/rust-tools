// Kevin Cantu
// Boyer-Moore string searching

use std;

// Function: findn_str_between
//
// Boyer-Moore string search, which returns
// up to `nn` byte positions of matched substrings
fn findn_str_between (haystack: str, needle: str, nn: uint,
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

   // simplify the table referencing
   let getShift = fn@(pos: uint) -> uint {
/*
      assert needle != "";
      assert 0u <= pos;
      assert       pos < nlen;
*/
      let charShift = ct[needle[pos] as uint];
      let prefShift = pt[nlen - pos];

      std::io::println(#fmt("<c%u p%u>", charShift, prefShift));

      if charShift > prefShift {
         ret charShift;
      } else {
         ret prefShift;
      }
   };

   let getCharShift = fn@(pos: uint) -> uint {
      let charShift = ct[needle[pos] as uint];
      std::io::println(#fmt("<c%u>", charShift));

      ret charShift;
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

         //std::io::println(#fmt("%02x.@%u =? %02x.@%u",
         //                 needle[windowii] as uint,
         //                 windowii,
         //                 haystack[outerii+windowii] as uint,
         //                 outerii + windowii));

         // if still matching
         if needle[windowii] == haystack[outerii+windowii] {

            // if full match
            if windowii == 0u {
               std::io::println(#fmt("[pushing %u]", outerii));
               vec::push(results, outerii);

               if vec::len(results) >= nn {
                  ret results;
               } else {
                  outerii += nlen;
               }
            }
         } else {
            outerii +=
               if windowii == nlen - 1u {
                  // not matching yet
                  //getCharShift(nlen - windowii)
                  // PENDING: is this offset wrong?
                  //          Why does it always return 4 in the utf8_B test?
                  getCharShift(nlen - windowii)
               } else {
                  // was partial match
                  getShift(nlen - windowii)
               };

            break;
         }
      }
   }

   ret results;
}

fn char_table (needle: str) -> [uint] {
   let len = str::len(needle);
   let mm  = vec::init_elt_mut(255u, len);

   let jj = len - 1u; // drop the last byte

   assert jj >= 0u;
   assert jj < str::len(needle);

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

fn prefix_table (needle: str) -> [uint] {
   let needle_ = str::bytes(needle);

   let lim   = vec::len(needle_);
   assert 0u < lim;

   let mm  = vec::init_elt_mut(lim, lim); // FIXME: is this an OK default?

   // step to larger suffixes
   let sii = 0u;
   while sii < lim {

      // tail of the needle we seek
      let suffix      = vec::slice(needle_, lim - sii,      lim);
      let suffix_plus = vec::slice(needle_, lim - sii - 1u, lim);
      let slen = vec::len(suffix);

      // step to smaller prefixes
      let pii = lim - 1u;
      while pii > 0u {

         // a prefix of the needle that might be matched by
         // a partial match of a suffix
         // (which we might want to jump to)
         let prefix = vec::slice(needle_, 0u, pii);
         let plen = vec::len(prefix);

         //let msg  = "<"+suffix+"("+#fmt("%u",sii)+")";
         //let msg2 = prefix+"("+#fmt("%u",pii)+")>";
         //let msg  = "<suf("+#fmt("%u",sii)+")";
         //let msg2 = "pref("+#fmt("%u",pii)+")>";
         //std::io::println(msg + " × " + msg2);

         // suffix might be fully matched
         let is_suffix_matched =
            vec::ends_with(prefix, suffix)
            && !vec::ends_with(prefix, suffix_plus)
            &&  slen < plen;

         // prefix is bigger than suffix, only tail can match
         let is_suffix_partially_matched = 
            plen <= slen
            && vec::ends_with(suffix, prefix);

         if is_suffix_matched || is_suffix_partially_matched {
            // if we haven't set it yet, set it now
            // (besides default)
            if mm[sii] == lim {
               mm[sii] = lim-pii;
               //std::io::println(#fmt("%u => %u", kk, vv));
            }
         }

         pii -= 1u;
      }

      // no matching prefix
      // if we haven't set it yet, set it now
      // (besides default)
      if mm[sii] == lim {
         mm[sii] = lim-pii;
         //std::io::println(#fmt("%u => %u", kk, vv));
      }

      sii += 1u;
   }

   ret vec::from_mut(mm);
}

//#[test]
fn test_char_table () {
   let ct = char_table("ANPANMAN");
   assert 1u == ct['A' as uint];
   assert 2u == ct['M' as uint];
   assert 3u == ct['N' as uint];
   assert 5u == ct['P' as uint];
   assert str::len("ANPANMAN") == ct['z' as uint];
}

//#[test]
fn test_prefix_table_ascii() {
   let pt = prefix_table("ANPANMAN");
                            //      ... 8

   assert 1u == pt[0u]; //        (n)
   assert 8u == pt[1u]; //       (a)n
   assert 3u == pt[2u]; //      (m)an
   assert 6u == pt[3u]; //     (n)man
   assert 6u == pt[4u]; //    (a)nman
   assert 6u == pt[5u]; //   (p)anman
   assert 6u == pt[6u]; //  (n)panman
   assert 6u == pt[7u]; // (a)npanman
   //assert 0u == pt.get(8u); // fail
}

//#[test]
fn test_prefix_table_utf8() {
   let pt = prefix_table("ประเ");

   assert  1u == pt[0u];
   assert 12u == pt[3u];
   assert 12u == pt[6u];
   assert 12u == pt[9u];
}

fn findn_str(haystack: str, needle: str, nn: uint) -> [uint] {
   findn_str_between(haystack, needle, nn, 0u, str::len(haystack))
}

fn find_str_(haystack: str, needle: str) -> option<uint> {
   let found = findn_str(haystack, needle, 1u);
   
   alt vec::len(found) {
      0u { option::none }
      xx { option::some(found[0u]) }
   }
}

//#[test]
fn test_findn_str() {
   assert [] == findn_str("banana", "apple pie", 1u);
   assert (findn_str("abcxxxxxx", "abc", 1u) == [0u]);
   assert (findn_str("xxxabcxxx", "abc", 1u) == [3u]);
   assert (findn_str("xxxxxxabc", "abc", 1u) == [6u]);
   assert (findn_str("xxxabcabc", "abc", 1u) == [3u]);
   assert (findn_str("xxxabcabc", "abc", 5u) == [3u, 6u]);
   assert (findn_str("xxxabcxabc", "abc", 5u) == [3u, 7u]);
   assert (findn_str("xxxabcxxabc", "abc", 5u) == [3u, 8u]);
}

//#[test]
fn test_find_strX_ascii() {
  assert (find_str_("banana", "apple pie") == option::none);
  assert (find_str_("", "") == option::none);
  assert (find_str_("abcxxxxxx", "abc") == option::some(0u));
  assert (find_str_("xxxabcxxx", "abc") == option::some(3u));
  assert (find_str_("xxxxxxabc", "abc") == option::some(6u));
}

//#[test]
fn test_find_strX_utf8() {
  let data = "ประเทศไทย中华Việt Nam";
  assert (find_str_(data, "ไท华") == option::none);
  assert (find_str_(data, "")     == option::some( 0u));
  assert (find_str_(data, "ประเ") == option::some( 0u));
  assert (find_str_(data, "ระ") == option::some(3u));
  assert (find_str_(data, "ย中华") == option::some(24u));
}

#[test]
fn test_find_strX_utf8_B() {
// PENDING
   let data = "ประเทศไทย中华Việt Nam";
   assert (find_str_(data, "ะเ")   == option::some( 6u));  // why does this shift 4 so much?
   assert (find_str_(data, "ศไทย中华") == option::some(15u));
   assert (find_str_(data, "ไทย中华") == option::some(18u));
   assert (find_str_(data, "中华") == option::some(27u));
}

#[test]
fn hmm() {
   let ct = char_table("ะเ"); //e0b8b0 e0b980
   assert 2u == ct[0x_e0_u];
   assert 4u == ct[0x_b8_u];
   assert 3u == ct[0x_b0_u];
   assert 2u == ct[0x_e0_u];
   assert 1u == ct[0x_b9_u];
   assert 6u == ct[0x_80_u];

   assert 6u == ct["ะเ"[5u]]
}


