// Kevin Cantu
// Boyer-Moore string searching

use std;

// Function: findn_bytes
//
// Boyer-Moore string search, which returns
// up to `nn` byte positions of matched substrings
fn findn_bytes (needle: str, haystack: str, nn: uint) -> [uint] {
   std::io::println("needle: " + needle);
   std::io::println("haystack: " + haystack);

   let results = [];

   if str::len_bytes(haystack) == 0u {
      ret results;
   }

   if str::len_bytes(needle) == 0u {
      vec::push(results, 0u);
      ret results;
   }

   if str::len_bytes(haystack) < str::len_bytes(needle) {
      ret results;
   }

   // generate the tables
   let ct = char_table(needle);
   std::io::println("hmmm");
   let pt = prefix_table(needle);
   std::io::println("xxx");


   // simplify the table referencing
   let getShift = fn@(pos: uint) -> uint {
      let charShift, prefShift; // also: ct, pt, needle

      assert needle != "";
      assert pos >= 0u;
      assert pos < str::len_bytes(needle);

      alt ct.find(needle[pos] as uint) {
         option::none { charShift = str::len_bytes(needle);}
         option::some(x) { charShift = x;}
      }

      let offset = str::len_bytes(needle) - pos;

      std::io::print(#fmt("[offset: %u]", offset));

      assert offset >= 0u;
      assert offset < str::len_bytes(needle);

      alt pt.find(offset) {
         option::none { fail "something is out of range" }
         option::some(x) { prefShift = x;}
      }

      ret greaterOf(charShift, prefShift);
   };

   let hii = 0u;
   let njj = 0u;

   while hii < str::len_bytes(haystack) {
      std::io::print(".");

      njj = str::len_bytes(needle);

      // reverse through needle
      while 0u < njj {
         njj -= 1u;

         std::io::print(#fmt(" +{%u,%u}", hii,njj));

         if njj+hii >= str::len_bytes(needle) {
            ret results;
         }

         // if still matching
         if needle[njj] == haystack[hii+njj] {
            std::io::print("(= " + #fmt("%u", njj) + ")");

            // if full match
            if njj == 0u {
               vec::push(results, njj);
               if vec::len(results) >= nn { ret results; }
            }

            hii += 1u;

         // if partial match
         } else {
            std::io::print("(| " + #fmt("%u", njj) + ")");

            let shift = getShift(njj);

            hii += shift;
            njj = str::len_bytes(needle);
         }

         std::io::println("");

      }

      //hii += 1u;
   }

   ret results;
}

fn char_table (needle: str) -> std::map::map<uint, uint> {
   let mm: std::map::map<uint, uint> = std::map::new_uint_hash();
   let len = str::len_bytes(needle);

   let jj = len - 1u; // drop the last byte

   assert jj >= 0u;
   assert jj < str::len_bytes(needle);

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

//#[test]
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


   let prefs = vec::reversed(prefixes(needle)); // largest first
   let sufs  = vec::reversed(suffixes(needle)); // smallest first
   let jj    = 0u;
   let kk    = 0u;
   let lim   = str::len_bytes(needle);

   vec::iter(sufs) {|suf|
      kk = 0u;

      vec::iter(prefs) {|pref|

         assert jj+1u >= 0u;
         assert jj+1u < str::len_bytes(suf); // AHA!

         // combine this logic

         if str::len_bytes(pref) > str::len_bytes(suf) {
            if str::ends_with(pref, suf) {
               if !str::ends_with(pref, sufs[jj+1u]) {
                  if ! mm.contains_key(jj)
                  {
                     mm.insert(jj, kk);
                  }
               }
            }
         }

         if str::len_bytes(pref) <= str::len_bytes(suf) {

            if str::ends_with(suf, pref) // partial
            {

               if ! mm.contains_key(jj)
               {
                  mm.insert(jj, kk);
               }
            }
         }


         kk += 1u;
      }

      // for this suffix jj, if no matching prefix, where did we fall out?
      // i.e., what is the last prefix that was still partially matching,
      // e.g., for suffix "man", the prefix "an" works, and is +6...

      if ! mm.contains_key(jj)
      {
         mm.insert(jj, kk);
      }

      jj += 1u;
   }

   ret mm;
}

//#[test]
fn test_prefix_table() {
   let pt = prefix_table("ANPANMAN");
                        //     ... 8

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
   assert str::len_bytes(ss) > 0u;

   let vv = [];
   let sz = 0u;
   vec::push(vv, "");
   str::bytes_iter(ss) {|_bb|
      sz += 1u;
      vec::push(vv, str::unsafe::slice_bytes(ss, 0u, sz));
   }
   ret vv;
}

//#[test]
fn test_prefs() {
   assert ["", "a", "ab", "abc", "abcd"] == prefixes("abcd");
}


// suffixes, largest first
fn suffixes(ss: str) -> [str] unsafe {
   assert str::len_bytes(ss) > 0u;

   let vv = [];
   let sz = 0u;
   let lim = str::len_bytes(ss);
   str::bytes_iter(ss) {|_bb|
      vec::push(vv, str::unsafe::slice_bytes(ss, sz, lim));
      sz += 1u;
   }
   vec::push(vv, "");
   ret vv;
}

//#[test]
fn test_sufs() {
   assert ["abcd", "bcd", "cd", "d", ""] == suffixes("abcd");
}

fn greaterOf(a: uint, b: uint) -> uint {
   if a > b { ret a; } else { ret b; }
}

//#[test]
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

//#[test]
fn test_findn() {
   assert [] == findn_bytes("banana", "apple pie", 1u);
}

//#[test]
fn test_find_bytes_A() {
  // byte positions
  assert (find_bytes_("banana", "apple pie") == option::none);
}

//#[test]
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


