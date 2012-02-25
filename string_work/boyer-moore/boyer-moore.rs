// Kevin Cantu
// Boyer-Moore string searching

use std;

// Function: findn_bytes
//
// Boyer-Moore string search, which returns
// up to `nn` byte positions of matched substrings
fn findn_bytes (needle: str, haystack: str, nn: uint) -> [uint] {

   let results = [];

   // empties
   if str::len(haystack) == 0u {
      ret results;
   }

   if str::len(needle) == 0u {
      vec::push(results, 0u);
      ret results;
   }

   if str::len(haystack) < str::len(needle) {
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
      assert pos < str::len(needle);

      alt ct.find(needle[pos] as uint) {
         option::none { charShift = str::len(needle);}
         option::some(x) { charShift = x;}
      }

      let offset = str::len(needle) - pos;


      assert offset >= 0u;
      assert offset < str::len(needle);

      alt pt.find(offset) {
         option::none { fail "something is out of range" }
         option::some(x) { prefShift = x;}
      }

      ret greaterOf(charShift, prefShift);
   };

   let hii = 0u;
   let njj = 0u;

   while hii < str::len(haystack) {

      njj = str::len(needle);

      // reverse through needle
      while 0u < njj {
         njj -= 1u;

         if njj+hii >= str::len(needle) {
            ret results;
         }

         // if still matching
         if needle[njj] == haystack[hii+njj] {

            // if full match
            if njj == 0u {
               vec::push(results, hii);
               log(warn, results);
               if vec::len(results) >= nn { ret results; }
            }

            hii += 1u;

         // if partial match
         } else {
            let shift = getShift(njj);

            hii += shift;
            njj = str::len(needle);
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
   let mm: std::map::map<uint, uint> = std::map::new_uint_hash();

   // WAIT, HOW IS THIS ALLOWED TO MUTATE mm?
   let fill = fn@(kk: uint, vv: uint) {
      if ! mm.contains_key(kk) {
         mm.insert(kk, vv);
         //std::io::println(#fmt("%u => %u", kk, vv));
      }
   };

   let lim   = str::len(needle);
   assert 0u < lim;

   // step to larger suffixes
   let sii = 0u;
   while sii < lim {

      // tail of the needle we seek
      let suffix      = str::slice(needle, lim - sii,      lim);
      let suffix_plus = str::slice(needle, lim - sii - 1u, lim);
      let slen = str::len(suffix);

      // step to smaller prefixes
      let pii = lim - 1u;
      while pii > 0u {

         // a prefix of the needle that might be matched by
         // a partial match of a suffix
         // (which we might want to jump to)
         let prefix = str::slice(needle, 0u, pii);

         //let msg  = "<"+suffix+"("+#fmt("%u",sii)+")";
         //let msg2 = prefix+"("+#fmt("%u",pii)+")>";
         //std::io::println(msg + " × " + msg2);

         // suffix might be fully matched
         let is_suffix_matched =
            str::ends_with(prefix, suffix)
            && !str::ends_with(prefix, suffix_plus)
            &&  slen < str::len(prefix);

         // prefix is bigger than suffix, only tail can match
         let is_suffix_partially_matched = 
            str::len(prefix) <= str::len(suffix)
            && str::ends_with(suffix, prefix);

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
fn test_prefix_table() {
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

////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

// prefixes, smallest first
fn prefixes(ss: str) -> [str] unsafe {
   let vv = [];

   vec::push(vv, "");
   str::chars_iteri(ss) {|ii, _bb|
      vec::push(vv, str::slice(ss, 0u, ii+1u));
   }

   ret vv;
}

#[test]
fn test_prefs() {
   assert ["", "a", "ab", "abc", "abcd"] == prefixes("abcd");
   assert [""] == prefixes("");
}


// suffixes, largest first
fn suffixes(ss: str) -> [str] unsafe {
   let vv = [];
   let lim = str::len(ss);

   str::chars_iteri(ss) {|ii, _bb|
      vec::push(vv, str::slice(ss, ii, lim));
   }
   vec::push(vv, "");

   ret vv;
}

#[test]
fn test_sufs() {
   assert ["abcd", "bcd", "cd", "d", ""] == suffixes("abcd");
   assert [""] == suffixes("");
}

fn greaterOf(a: uint, b: uint) -> uint {
   if a > b { ret a; } else { ret b; }
}

#[test]
fn test_greaterOf() {
   assert greaterOf(15u, 17u) == 17u;
   assert greaterOf(17u, 15u) == 17u;
}

fn find_bytes_(needle: str, haystack: str) -> option<uint> {
   let found = findn_bytes(needle, haystack, 1u);
   
   alt vec::len(found) {
      0u { option::none }
      xx { option::some(found[0u]) }
   }
}

#[test]
fn test_findn() {
   assert [] == findn_bytes("banana", "apple pie", 1u);
}

#[test]
fn test_find_bytes_A() {
  // byte positions
  assert (find_bytes_("banana", "apple pie") == option::none);
}

#[test]
fn test_find_bytes_B() {
  assert (find_bytes_("", "") == option::none);
}

#[test]
fn test_find_bytes_C() {
  let data = "ประเทศไทย中华Việt Nam";
//  assert (find_bytes_("", data)     == option::some( 0u));
// PENDING
  assert (find_bytes_("ประเ", data) == option::some( 0u));
//  assert (find_bytes_("ะเ", data)   == option::some( 6u));
//  assert (find_bytes_("中华", data) == option::some(27u));
//  assert (find_bytes_("ไท华", data) == option::none);
}


