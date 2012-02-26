// Kevin Cantu
// Boyer-Moore string searching

use std;

// Function: findn
//
// Boyer-Moore string search, which returns
// up to `nn` byte positions of matched substrings
fn findn (needle: str, haystack: str, nn: uint) -> [uint] {

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
      let charShift, prefShift; // also: ct, pt, needle

      assert needle != "";
      assert pos >= 0u;
      assert pos < nlen;

      alt ct.find(needle[pos] as uint) {
         option::none    { charShift = nlen;}
         option::some(x) { charShift = x;}
      }

      let offset = nlen - pos;


      assert offset >= 0u;
      assert offset < nlen;

      alt pt.find(offset) {
         option::none { fail "something is out of range" }
         option::some(x) { prefShift = x;}
      }

      //std::io::println(#fmt("<charShift: %u, prefShift: %u>", charShift, prefShift));
      ret greaterOf(charShift, prefShift);
   };

   // step up through the haystack
   let outerii = 0u;
   while outerii < hlen - nlen + 1u {

      // step back through needle
      let windowii = nlen;
      while 0u < windowii
            && (outerii + (windowii - 1u)) < hlen  // don't exceed haystack
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
               //std::io::println(#fmt("[pushing %u]", outerii));
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
                  1u
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

fn char_table (needle: str) -> std::map::map<uint, uint> {
   let mm: std::map::map<uint, uint> = std::map::new_uint_hash();
   let len = str::len(needle);

   let jj = len - 1u; // drop the last byte

   assert jj >= 0u;
   assert jj < str::len(needle);

   // from last-1 to first
   while jj > 0u {
      jj -= 1u;
      if ! mm.contains_key(needle[jj] as uint) {
         // store reverse indexes
         mm.insert(needle[jj] as uint, len - 1u - jj);
      }
   }

   ret mm;
}

#[test]
fn test_char_table () {
   let ct = char_table("ANPANMAN");
   assert 1u == ct.get('A' as uint);
   assert 2u == ct.get('M' as uint);
   assert 3u == ct.get('N' as uint);
   assert 5u == ct.get('P' as uint);
   assert option::none == ct.find('z' as uint);
}

fn prefix_table (needle: str) -> std::map::map<uint, uint> {
   let needle_ = str::bytes(needle);
   let mm: std::map::map<uint, uint> = std::map::new_uint_hash();

   // WAIT, HOW IS THIS ALLOWED TO MUTATE mm?
   let fill = fn@(kk: uint, vv: uint) {
      if ! mm.contains_key(kk) {
         mm.insert(kk, vv);
         //std::io::println(#fmt("%u => %u", kk, vv));
      }
   };

   let lim   = vec::len(needle_);
   assert 0u < lim;

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
            fill(sii, lim-pii);
         }

         pii -= 1u;
      }

      // no matching prefix
      fill(sii, lim-pii);

      sii += 1u;
   }

   ret mm;
}

#[test]
fn test_prefix_table_ascii() {
   let pt = prefix_table("ANPANMAN");
                            //      ... 8

   assert 1u == pt.get(0u); //        (n)
   assert 8u == pt.get(1u); //       (a)n
   assert 3u == pt.get(2u); //      (m)an
   assert 6u == pt.get(3u); //     (n)man
   assert 6u == pt.get(4u); //    (a)nman
   assert 6u == pt.get(5u); //   (p)anman
   assert 6u == pt.get(6u); //  (n)panman
   assert 6u == pt.get(7u); // (a)npanman
   //assert 0u == pt.get(8u); // fail
}

#[test]
fn test_prefix_table_utf8() {
   let pt = prefix_table("ประเ");

   assert  1u == pt.get(0u);
   assert 12u == pt.get(3u);
   assert 12u == pt.get(6u);
   assert 12u == pt.get(9u);
}

fn greaterOf(a: uint, b: uint) -> uint {
   if a > b { ret a; } else { ret b; }
}

#[test]
fn test_greaterOf() {
   assert greaterOf(15u, 17u) == 17u;
   assert greaterOf(17u, 15u) == 17u;
}

fn find_(needle: str, haystack: str) -> option<uint> {
   let found = findn(needle, haystack, 1u);
   
   alt vec::len(found) {
      0u { option::none }
      xx { option::some(found[0u]) }
   }
}

#[test]
fn test_findn() {
   assert [] == findn("banana", "apple pie", 1u);
   assert (findn("abc", "abcxxxxxx", 1u) == [0u]);
   assert (findn("abc", "xxxabcxxx", 1u) == [3u]);
   assert (findn("abc", "xxxxxxabc", 1u) == [6u]);
   assert (findn("abc", "xxxabcabc", 1u) == [3u]);
   assert (findn("abc", "xxxabcabc", 5u) == [3u, 6u]);
   assert (findn("abc", "xxxabcxabc", 5u) == [3u, 7u]);
   assert (findn("abc", "xxxabcxxabc", 5u) == [3u, 8u]);
}

#[test]
fn test_find_ascii() {
  assert (find_("banana", "apple pie") == option::none);
  assert (find_("", "") == option::none);
  assert (find_("abc", "abcxxxxxx") == option::some(0u));
  assert (find_("abc", "xxxabcxxx") == option::some(3u));
  assert (find_("abc", "xxxxxxabc") == option::some(6u));
}

#[test]
fn test_find_utf8() {
  let data = "ประเทศไทย中华Việt Nam";
  assert (find_("ไท华", data) == option::none);
  assert (find_("", data)     == option::some( 0u));
  assert (find_("ประเ", data) == option::some( 0u));
  assert (find_("ะเ", data)   == option::some( 6u));
  assert (find_("ระ", data) == option::some(3u));
  assert (find_("ศไทย中华", data) == option::some(15u));
  assert (find_("ไทย中华", data) == option::some(18u));
  assert (find_("ย中华", data) == option::some(24u));
  assert (find_("中华", data) == option::some(27u));
}


