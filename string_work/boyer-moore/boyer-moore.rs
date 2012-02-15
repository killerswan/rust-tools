// Kevin Cantu
// Boyer-Moore string searching

use std;

// Function: findn_bytes
//
// Boyer-Moore string search, which returns
// up to `nn` byte positions of matched substrings
fn findn_bytes (needle: str, haystack: str, nn: uint) -> [uint] {
   let results = [];
   let ct = char_table(needle);
   let pt = prefix_table(needle);

   log(error, ct);
   log(error, pt);

   let ii = 0u;
   let jj = 0u;

   while ii < str::len_bytes(haystack) {

      jj = str::len_bytes(needle) - 1u;

      // reverse through needle
      while 0u <= jj {

         // if still matching
         if needle[jj] == haystack[ii] {
            // if full match
            if jj == 0u {
               vec::push(results, jj);
               if vec::len(results) >= nn { ret results; }
            }

         // if partial match
         } else {
            let sufn = str::len_bytes(needle) - jj;

            let ctx;
            let ptx;

            alt ct.find(needle[sufn] as uint) {
               option::none { ctx = str::len_bytes(needle);}
               option::some(x) { ctx = x;}
            }

            alt pt.find(sufn) {
               option::none { fail "something is out of range" }
               option::some(x) { ptx = x;}
            }

            ii += greaterOf(ctx, ptx) - jj;
            jj = 0u;
         }


         jj -= 1u;
      }

      ii += 1u;
   }

   ret results;
}

fn char_table (needle: str) -> std::map::map<uint, uint> {
   let mm: std::map::map<uint, uint> = std::map::new_uint_hash();
   let len = str::len_bytes(needle);

   let jj = len - 1u; // drop the last byte

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

   let prefs = vec::reversed(prefixes(needle)); // largest first
   let sufs  = vec::reversed(suffixes(needle)); // smallest first
   let jj    = 0u;
   let kk    = 0u;
   let lim   = str::len(needle);

   while jj < lim {    // sufixes
      kk = 0u;

      while kk < lim { // prefixes
         let suf = sufs[jj];
         let pref = prefs[kk];


         if str::len_bytes(pref) > str::len_bytes(suf) {
            std::io::println("[" + pref + ", " + suf + "]");
            std::io::println(">");
            if str::ends_with(pref, suf) {
               std::io::println("(suf)");
               if !str::ends_with(pref, sufs[jj+1u]) {
                  std::io::println("(suf+1)");
               }
            }
         }

         if str::len_bytes(pref) > str::len_bytes(suf) &&
            str::ends_with(pref, suf) &&
           !str::ends_with(pref, sufs[jj+1u])
         {
            std::io::print("-");

            if ! mm.contains_key(jj)
            {
               std::io::println("*");
               mm.insert(jj, kk);
            }
         }

         kk += 1u;
      }
      jj += 1u;
   }

   ret mm
}

#[test]
fn test_prefix_table() {
   let pt = prefix_table("ANPANMAN");
   std::io::print("+");
   log(error, pt);
   assert 8u == pt.get(2u);
}

////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

// prefixes, smallest first
fn prefixes(ss: str) -> [str] unsafe {
   let vv = [];
   let sz = 0u;
   str::bytes_iter(ss) {|_bb|
      sz += 1u;
      vec::push(vv, str::unsafe::slice_bytes(ss, 0u, sz));
   }
   ret vv;
}

#[test]
fn test_prefs() {
   assert ["a", "ab", "abc", "abcd"] == prefixes("abcd");
}


// suffixes, largest first
fn suffixes(ss: str) -> [str] unsafe {
   let vv = [];
   let sz = 0u;
   let lim = str::len_bytes(ss);
   str::bytes_iter(ss) {|_bb|
      vec::push(vv, str::unsafe::slice_bytes(ss, sz, lim));
      sz += 1u;
   }
   ret vv;
}

#[test]
fn test_sufs() {
   assert ["abcd", "bcd", "cd", "d"] == suffixes("abcd");
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
   ret option::some(findn_bytes(needle, haystack, 1u)[0u]);
}

//#[test]
fn test_find_bytes() {
  // byte positions
  assert (find_bytes_("banana", "apple pie") == option::none);
  assert (find_bytes_("", "") == option::some(0u));

  let data = "ประเทศไทย中华Việt Nam";
  assert (find_bytes_(data, "")     == option::some(0u));
  assert (find_bytes_(data, "ประเ") == option::some( 0u));
  assert (find_bytes_(data, "ะเ")   == option::some( 6u));
  assert (find_bytes_(data, "中华") == option::some(27u));
  assert (find_bytes_(data, "ไท华") == option::none);
}


