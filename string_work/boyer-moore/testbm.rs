use std;
use boyermoore;
use meow;

fn main() {
   let bible = meow::bible();
   let phrase = "And leik, dont think ur all hot soup cuz you just read the FAQ.";
   let phrase2 = "And leik, dont think ur all hot soup cuz you just read the BANANAS.";

   //let ref = bind str::find_str(_, phrase);
   //let new = bind boyermoore::find_str_(_, phrase);

   //meow::compare_sweep_strings("ref^2", ref, ref, 0u, 500000u);
   //meow::compare_sweep_strings("ref vs. new", ref, new, 0u, 500000u);

   meow::time("simple ref", {|| str::find_str(phrase+phrase, phrase)});
   meow::time("simple new", {|| boyermoore::find_str_(phrase+phrase, phrase)});
   meow::time("bible ref", {|| str::find_str(bible, phrase)});
   meow::time("bible new", {|| boyermoore::find_str_(bible, phrase)});

   let ii = 0u;
   let N = 1000u;
   let bibleN = "";
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("bibleN ref", {|| str::find_str(        bibleN, phrase2)});
   meow::time("bibleN new", {|| boyermoore::find_str_(bibleN, phrase2)});
}


