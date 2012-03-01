use std;
use boyermoore;
use meow;

fn main() {
   let bible = meow::bible();
   //let phrase = "And leik, dont think ur all hot soup cuz you just read the FAQ.";
   let phrase = "And leik, dont think ur all hot soup cuz you just read the BANANAS.";
   let phrase2 = "x-ray-banana";

   //let ref = bind str::find_str(_, phrase);
   //let new = bind boyermoore::find_str(_, phrase);

   //meow::compare_sweep_strings("ref^2", ref, ref, 0u, 500000u);
   //meow::compare_sweep_strings("ref vs. new", ref, new, 0u, 500000u);

   meow::time("a0 ref", {||        str::find_str("hello", phrase)});
   meow::time("a0 new", {|| boyermoore::find_str("hello", phrase)});

   let xx = phrase+phrase;

   meow::time("a1 ref", {||        str::find_str(xx, "")});
   meow::time("a1 new", {|| boyermoore::find_str(xx, "")});

   let _xx = boyermoore::find_str(xx, phrase);

   let xx = phrase;

   meow::time("a3 ref", {||        str::find_str(xx, xx)});
   meow::time("a3 new", {|| boyermoore::find_str(xx, xx)});  // larger needle, slower start

   meow::time("a4 ref", {||        str::find_str("hello", "hello")});
   meow::time("a4 new", {|| boyermoore::find_str("hello", "hello")});

   let yy = "superxpalidocious exxpeealidocious";
   meow::time("a5 ref", {||        str::find_str(yy, yy)});
   meow::time("a5 new", {|| boyermoore::find_str(yy, yy)});  // larger needle, slower start

   meow::time("a ref", {||        str::find_str(xx, phrase2)});
   meow::time("a new", {|| boyermoore::find_str(xx, phrase2)});

   meow::time("b ref", {||        str::find_str(bible, "lulz")});
   meow::time("b new", {|| boyermoore::find_str(bible, "lulz")});

   meow::time("c ref", {||        str::find_str(bible, phrase)});
   meow::time("c new", {|| boyermoore::find_str(bible, phrase)});

   let yy = "superxpalidocious exxpeealidocioussuperxpalidocious exxpeealidocioussuperxpalidocious exxpeealidocious";
   meow::time("c1 ref", {||        str::find_str(bible, yy)});
   meow::time("c1 new", {|| boyermoore::find_str(bible, yy)});

   let ii = 0u;
   let N = 40u;
   let bibleN = "";
   str::reserve(bibleN, N * str::len(bibleN) + 1u);
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("d ref", {||        str::find_str(bibleN, phrase2)});
   meow::time("d new", {|| boyermoore::find_str(bibleN, phrase2)});

   let ii = 0u;
   let N = 80u;
   let bibleN = "";
   str::reserve(bibleN, N * str::len(bibleN) + 1u);
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("e ref", {||        str::find_str(bibleN, phrase2)});
   meow::time("e new", {|| boyermoore::find_str(bibleN, phrase2)});

   let ii = 0u;
   let N = 160u;
   let bibleN = "";
   str::reserve(bibleN, N * str::len(bibleN) + 1u);
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("f ref", {||        str::find_str(bibleN, phrase2)});
   meow::time("f new", {|| boyermoore::find_str(bibleN, phrase2)});
   meow::time("f2 ref", {||        str::find_str(bibleN, phrase)});
   meow::time("f2 new", {|| boyermoore::find_str(bibleN, phrase)});
}


