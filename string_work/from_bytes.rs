
use std;
use meow;

fn main () {
   meow::compare_sweep_u8vecs("unsafe vs safe from_bytes", str::unsafe_from_bytes, str::from_bytes, 10000u, 100000u);
   meow::compare_sweep_strings("unsafe vs safe from_bytes", {|x| str::unsafe_from_bytes(str::bytes(x))}, {|x| str::from_bytes(str::bytes(x))}, 100u, 100000u);
}

