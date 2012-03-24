use std;

#[test]
fn ascii() unsafe {
   let data = "ประเทศไทย中华Việt Nam";
   assert ! str::all(data, char::is_ascii);

   assert str::all("banana", char::is_ascii);

   let bytestr = str::unsafe::from_bytes([127u8, 0u8, 14u8]);
   assert str::all(bytestr, char::is_ascii);

   let messed_up = str::unsafe::from_bytes([127u8, 128u8, 255u8,0u8, 14u8]);
   assert ! str::all(messed_up, char::is_ascii);
}
