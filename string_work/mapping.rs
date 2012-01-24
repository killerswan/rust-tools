
use std;
use meow;

/*
Function: all

Return true if a predicate matches all characters

If the string contains no characters
*/
fn all(ss: str, ff: fn&(char) -> bool) -> bool {
    str::loop_chars(ss, ff)
}

/*
Function: map

Apply a function to each character
*/
fn map(ss: str, ff: fn&(char) -> char) -> str {
    let result = "";

    str::iter_chars(ss, {|cc|
        str::push_char(result, ff(cc));
    });

    ret result;
}

fn map_reserve(ss: str, ff: fn&(char) -> char) -> str {
   let result = "";
   str::reserve(result, str::byte_len(ss));

   str::iter_chars(ss, {|cc|
      str::push_char(result, ff(cc));
   });

   ret result;
}

#[test]
fn test_map () {
   assert "" == map("", char::to_upper);
   assert "YMCA" == map("ymca", char::to_upper);
}

#[test]
fn test_all () {
    assert true  == all("", char::is_uppercase);
    assert false == all("ymca", char::is_uppercase);
    assert true  == all("YMCA", char::is_uppercase);
    assert false == all("yMCA", char::is_uppercase);
    assert false == all("YMCy", char::is_uppercase);
}

fn main () {
   let ff = bind map(_,char::to_upper);
   let gg = bind map_reserve(_,char::to_upper);

   fn ff_ () { map(meow::sample_string(), char::to_upper); }
   fn gg_ () { map_reserve(meow::sample_string(), char::to_upper); }

   meow::compare("map vs. map_reserve", ff_, gg_);
   meow::compare_several("map vs. map_reserve", ff_, gg_);

   meow::compare_sweep_strings("map vs. map_reserve", ff, gg, 0u, 500000u);


/* EXAMPLE RUN:
   $ rustc -O meow.rc
   $ rustc -L . -O mapping.rs 
   $ ./mapping 
   meow: map vs. map_reserve:	 0.939 ms,  0.952 ms
   meow: map vs. map_reserve (avg):	 0.707 ms,  0.730 ms
   meow: Sweeping across strings of 0 to 500000 (5 tests per step)...
   meow: map vs. map_reserve (0):	 0.001 ms,  0.001 ms
   meow: map vs. map_reserve (50000):	12.584 ms, 12.204 ms
   meow: map vs. map_reserve (100000):	25.159 ms, 24.888 ms
   meow: map vs. map_reserve (150000):	37.548 ms, 37.828 ms
   meow: map vs. map_reserve (200000):	50.871 ms, 50.242 ms
   meow: map vs. map_reserve (250000):	62.277 ms, 62.541 ms
   meow: map vs. map_reserve (300000):	74.354 ms, 74.836 ms
   meow: map vs. map_reserve (350000):	87.435 ms, 87.176 ms
   meow: map vs. map_reserve (400000):	100.788 ms, 100.492 ms
   meow: map vs. map_reserve (450000):	110.858 ms, 111.869 ms
   meow: map vs. map_reserve (500000):	125.730 ms, 125.049 ms
*/
}


