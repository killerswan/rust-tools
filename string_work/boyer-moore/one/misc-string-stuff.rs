use std;

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


