// Kevin Cantu
// Boyer-Moore string searching

use std;

fn boyermoore (needle: str, haystack: str) -> uint {
   3u
}

// bad character shift
fn table2 (needle: str) -> std::map::map<uint, uint> {
   let mm: std::map::map<uint, uint> = std::map::new_uint_hash();
   let len = str::byte_len(needle);

   let jj = len - 2u; // drop the last byte

   // from last-1 to first
   while jj >= 0u {
      if ! mm.contains_key(needle[jj] as uint) {
         // store reverse indexes
         mm.insert(jj as uint, len - 1u - jj);
      }
      jj -= 1u;
   }

   ret mm;
}


