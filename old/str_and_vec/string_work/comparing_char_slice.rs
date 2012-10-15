// Kevin Cantu

use std;

fn windowed_by (slicefn: fn&(str,uint,uint)->str, nn: uint, ss: str) -> [str]
{
   let ww = [];
   let len = str::char_len(ss);

   assert 1u <= nn;
   assert nn <= len;

   let ii = 0u;
   while ii+nn <= len
   {
      let w = slicefn( ss, ii, ii+nn );
      vec::push(ww,w);
      ii += 1u;
   }

   ret ww;
}


fn word_of_god () -> str
{
   std::io::println ("Loading the lolcat bible...");
   let path = "./lolcat/LOLCatBible_2012-01-04.xml";
   let bible = std::io::read_whole_file (path);
   let bible_ = str::unsafe_from_bytes (result::get (bible));
   ret str::char_slice(bible_, 0u, 20000u);
}

fn main ()
{
   let meow = word_of_god();

   let t0 = std::time::precise_time_s();
   let _v = windowed_by(str::char_slice_orig, 100u, meow);
   std::io::println (".");
   let _v = windowed_by(str::char_slice_orig, 1000u, meow);
   std::io::println (".");
   let _v = windowed_by(str::char_slice_orig, 10000u, meow);
   std::io::println (".");
   let t1 = std::time::precise_time_s();

   std::io::println("str::char_slice_orig -> " + #fmt("%06.3f sec", t1 - t0));



   let t0 = std::time::precise_time_s();
   let _v = windowed_by(str::char_slice, 100u, meow);
   std::io::println (".");
   let _v = windowed_by(str::char_slice, 1000u, meow);
   std::io::println (".");
   let _v = windowed_by(str::char_slice, 10000u, meow);
   std::io::println (".");
   let t1 = std::time::precise_time_s();

   std::io::println("str::char_slice -> " + #fmt("%06.3f sec", t1 - t0));

}


