
use std;

// FIXME: make it fast (use Boyer-Moore, etc.)
fn split_str(ss: str, sep: str) -> [str] unsafe {
   // unsafe is justified: we are splitting
   // UTF-8 with UTF-8, so the results will be OK
   
   let sep_len = str::byte_len(sep);
   assert sep_len > 0u;

   let vv: [str] = [];

   let start = 0u, start_match = 0u, current = 0u, matching = 0u;

   str::bytes_iter(ss) {|cc|
      if sep[matching] == cc {
         matching += 1u;
      } else {
         start_match += 1u;
      }
      
      if matching == sep_len {
         // found a separator
         // push whatever is before it, including ""
         vec::push(vv, str::unsafe::slice_bytes(ss, start, start_match));

         // reset cursors and counters
         start = current + 1u;
         start_match = current + 1u;
         matching = 0u;
      }

      current += 1u;
   }

   // whether we have a "", or something meaningful, push it
   vec::push(vv, str::unsafe::slice_bytes(ss, start, current));
   ret vv;
}

#[test]
fn slicetest () {
   let x = "ab";
   assert "" == str::slice(x, 0u,0u);
}

#[test]
fn aa () {
  let x = "abc::hello::there";
  assert str::split_str(x, "::") == ["abc", "hello", "there"];
}

#[test]
fn aa_ () {
  let x = "abc::hello::there";
  assert split_str(x, "::") == ["abc", "hello", "there"];
}

#[test]
fn bb () {
  let y = "::hello::there";
  assert str::split_str(y, "::") == ["", "hello", "there"];
}

#[test]
fn bb_ () {
  let y = "::hello::there";
  assert split_str(y, "::") == ["", "hello", "there"];
}

#[test]
fn cc () {
  let y = "hello::there::";
  assert str::split_str(y, "::") == ["hello", "there", ""];
}

#[test]
fn cc_ () {
  let y = "hello::there::";
  assert split_str(y, "::") == ["hello", "there", ""];
}

#[test]
fn dd () {
  let data = "ประเทศไทย中华Việt Nam";
  assert ["ประเทศไทย", "Việt Nam"]
      == str::split_str (data, "中华");
}

#[test]
fn dd_ () {
  let data = "ประเทศไทย中华Việt Nam";
  assert ["ประเทศไทย", "Việt Nam"]
      == split_str (data, "中华");
}

#[test]
fn ee () {
  assert ["", "XXX", "YYY", ""]
      == str::split_str("zzXXXzzYYYzz", "zz");
}

#[test]
fn ee_ () {
  assert ["", "XXX", "YYY", ""]
      == split_str("zzXXXzzYYYzz", "zz");
}

#[test]
fn ff () {
  assert ["zz", "zYYYz"]
      == str::split_str("zzXXXzYYYz", "XXX");
}

#[test]
fn ff_ () {
  assert ["zz", "zYYYz"]
      == split_str("zzXXXzYYYz", "XXX");
}

#[test]
fn gg () {
  assert ["", "XXX", "YYY", ""] == str::split_str(".XXX.YYY.", ".");
}

#[test]
fn gg_ () {
  assert ["", "XXX", "YYY", ""] == split_str(".XXX.YYY.", ".");
}

#[test]
fn hh () {
  assert [""] == str::split_str("", ".");
}

#[test]
fn hh_ () {
  assert [""] == split_str("", ".");
}

#[test]
fn ii () {
  assert ["",""] == str::split_str("zz", "zz");
}

#[test]
fn ii_ () {
  assert ["",""] == split_str("zz", "zz");
}

#[test]
fn jj () {
  assert ["ok"] == str::split_str("ok", "z");
}

#[test]
fn jj_ () {
  assert ["ok"] == split_str("ok", "z");
}

#[test]
fn kk () {
  assert ["","z"] == str::split_str("zzz", "zz");
}

#[test]
fn kk_ () {
  assert ["","z"] == split_str("zzz", "zz");
}

#[test]
fn ll () {
  assert ["","","z"] == str::split_str("zzzzz", "zz");
}

#[test]
fn ll_ () {
  assert ["","","z"] == split_str("zzzzz", "zz");
}

//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
#[test]
fn test_split () {
  let data = "ประเทศไทย中华Việt Nam";
  assert ["ประเทศไทย中", "Việt Nam"]
      == str::split (data, {|cc| cc == '华'});

  assert ["", "", "XXX", "YYY", ""]
      == str::split("zzXXXzYYYz", char::is_lowercase);

  assert ["zz", "", "", "z", "", "", "z"]
      == str::split("zzXXXzYYYz", char::is_uppercase);

  assert ["",""] == str::split("z", {|cc| cc == 'z'});
  assert [""] == str::split("", {|cc| cc == 'z'});
  assert ["ok"] == str::split("ok", {|cc| cc == 'z'});
}

#[test]
fn test_split_chars () {
  let data = "ประเทศไทย中华Việt Nam";
  assert ["ประเทศไทย中", "Việt Nam"]
      == str::split_chars(data, '华');

  assert ["", "", "XXX", "YYY", ""]
      == str::split_chars("zzXXXzYYYz", 'z');
  assert ["",""] == str::split_chars("z", 'z');
  assert [""] == str::split_chars("", 'z');
  assert ["ok"] == str::split_chars("ok", 'z');
}
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////
