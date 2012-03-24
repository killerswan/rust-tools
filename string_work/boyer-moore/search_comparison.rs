use std;
use meow;

fn BM(haystack: str, needle: str) -> [uint] {
   str::boyer_moore_search(haystack, needle,
                           str::len(haystack),
                           0u, str::len(haystack))
}

fn simple(haystack: str, needle: str) -> [uint] {
   str::simple_search(haystack, needle,
                      str::len(haystack),
                      0u, str::len(haystack))
}

fn compareHN(hlen: uint, nlen: uint) -> float {
   // some strings to test
   let generator = rand::rng();
   let needle   = generator.gen_str(nlen);
   let haystack = generator.gen_str(hlen);

   // run each
   //let sT  = meow::measure_time({|| sim_found = simple(haystack, needle);});
   //let bmT = meow::measure_time({|| bm_found  = BM(haystack, needle);});
   let sT  = meow::measure_time({|| simple(haystack, needle)});
   let bmT = meow::measure_time({|| BM(haystack, needle)});

   // they've gotta be the same
   //assert sim_found == bm_found;
      
   ret bmT as float / sT as float;
}

fn main() {
/*
   let bible = meow::bible();
   //let phrase = "And leik, dont think ur all hot soup cuz you just read the FAQ.";
   let phrase = "And leik, dont think ur all hot soup cuz you just read the BANANAS.";
   let phrase2 = "x-ray-banana";


   meow::time("c simple", {|| simple(bible, phrase)});
   meow::time("c BM", {||    BM(bible, phrase)});

   let yy = "superxpalidocious exxpeealidocioussuperxpalidocious exxpeealidocioussuperxpalidocious exxpeealidocious";
   meow::time("c1 simple", {|| simple(bible, yy)});
   meow::time("c1 BM", {||    BM(bible, yy)});

   let mut ii = 0u;
   let N = 40u;
   let mut bibleN = "";
   str::reserve(bibleN, N * str::len(bibleN) + 1u);
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("d simple", {|| simple(bibleN, phrase2)});
   meow::time("d BM", {||    BM(bibleN, phrase2)});
*/

/*
   let loremipsum = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris laoreet consequat fringilla. Quisque venenatis lacinia ipsum non rhoncus. Donec adipiscing fringilla erat, a pretium purus rhoncus quis. Nunc rhoncus dui at ipsum bibendum tincidunt. Etiam eu lorem nulla. Nulla id est id augue blandit vestibulum. Nullam eleifend gravida feugiat. Aliquam vitae urna arcu. Ut sed enim dui, a suscipit sapien.

Ut sollicitudin, metus id malesuada dapibus, quam velit ultrices nibh, at feugiat diam tellus eget risus. Vestibulum in ultrices enim. Nulla at est molestie augue hendrerit tempor sed in purus. Mauris non lorem libero, id faucibus odio. Proin feugiat magna id diam laoreet eu dapibus sapien eleifend. Nunc imperdiet auctor hendrerit. Curabitur porta tempus quam, vel scelerisque elit feugiat ac. Vestibulum feugiat bibendum massa in dapibus. Quisque lacinia porttitor turpis et auctor. Ut quis tortor vitae ligula faucibus euismod nec eu risus. Nullam blandit, risus at consectetur dapibus, velit nisl tincidunt tortor, sed varius risus neque sit amet purus. In diam nisl, facilisis tristique convallis sit amet, posuere congue augue. Vestibulum malesuada, tellus vitae mattis laoreet, nulla felis posuere libero, et blandit eros erat at lacus. Morbi id placerat magna. Nulla lacus magna, gravida sit amet elementum et, posuere nec sapien. Maecenas vitae felis magna, fringilla facilisis massa.

Ut porta, ligula id blandit volutpat, felis sapien ornare sapien, a auctor felis nunc in risus. Etiam in erat at tellus rutrum euismod quis at quam. Proin massa mauris, auctor in tempor eu, cursus quis metus. Suspendisse et dolor quam. Nam id neque eget lorem interdum ultricies. Phasellus aliquet facilisis tortor ac pellentesque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Suspendisse sed porta purus. Donec consectetur imperdiet tortor et sagittis. Ut pretium vulputate tortor eget ullamcorper. Nullam a urna sollicitudin quam mollis fermentum eu id enim. Mauris pharetra blandit ultricies. In ligula justo, rhoncus et tempus vitae, rutrum non neque.

In accumsan pulvinar lorem sed sollicitudin. Donec justo sem, aliquet at posuere ut, tempus at est. Quisque mattis scelerisque felis ac semper. Quisque nec tristique orci. Donec id diam ut ipsum auctor hendrerit non non sem. Vivamus non erat eros, at vulputate dolor. Duis vehicula, neque sed rhoncus ornare, ante est placerat erat, non luctus justo libero quis mauris. Pellentesque sollicitudin dignissim faucibus. Nunc arcu urna, hendrerit at sagittis nec, dapibus eu leo. Sed eget libero nec risus aliquet hendrerit ullamcorper vel sem. Vestibulum eleifend elit sed tellus tempor elementum. Maecenas eu nibh quis tortor pretium ullamcorper id vel nibh. Pellentesque viverra, nulla vitae condimentum pretium, enim nisl euismod odio, eget vulputate velit justo in lorem.

Nunc eget leo ipsum. Nulla facilisi. Nam adipiscing justo id nisl aliquam at posuere nulla pharetra. Pellentesque fringilla fringilla placerat. Nam pulvinar vehicula augue, a vestibulum turpis rutrum eu. Curabitur semper viverra elit id ultricies. Nunc nibh lacus, congue at imperdiet eget, lacinia at lorem. Aliquam erat volutpat. Pellentesque malesuada ultrices arcu, eu dignissim nisi consectetur at. Nam tristique justo molestie lorem pellentesque vel consectetur augue hendrerit.
";


   meow::time("lorem ipsum (400) in bible (simple)",
              {|| simple(bible, str::slice(loremipsum, 0u, 400u))});
   meow::time("lorem ipsum (400) in bible (BM)   ",
              {||    BM(bible, str::slice(loremipsum, 0u, 400u))});


   let mut ii = 0u;
   let N = 40u;
   let mut bibleN = "";
   str::reserve(bibleN, N * str::len(bibleN) + 1u);
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("lorem ipsum (100) in bible x40 (simple)",
              {|| simple(bibleN, str::slice(loremipsum, 0u, 100u))});
   meow::time("lorem ipsum (100) in bible x40 (BM)   ",
              {||    BM(bibleN, str::slice(loremipsum, 0u, 100u))});

   meow::time("lorem ipsum (400) in bible x40 (simple)",
              {|| simple(bibleN, str::slice(loremipsum, 0u, 400u))});
   meow::time("lorem ipsum (400) in bible x40 (BM)   ",
              {||    BM(bibleN, str::slice(loremipsum, 0u, 400u))});


   let mut ii = 0u;
   let N = 160u;
   let mut bibleN = "";
   str::reserve(bibleN, N * str::len(bibleN) + 1u);
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("lorem ipsum (100) in bible x160 (simple)",
              {|| simple(bibleN, str::slice(loremipsum, 0u, 100u))});
   meow::time("lorem ipsum (100) in bible x160 (BM)   ",
              {||    BM(bibleN, str::slice(loremipsum, 0u, 100u))});

   meow::time("lorem ipsum (400) in bible x160 (simple)",
              {|| simple(bibleN, str::slice(loremipsum, 0u, 400u))});
   meow::time("lorem ipsum (400) in bible x160 (BM)   ",
              {||    BM(bibleN, str::slice(loremipsum, 0u, 400u))});


   let mut ii = 0u;
   let N = 260u;
   let mut bibleN = "";
   str::reserve(bibleN, N * str::len(bibleN) + 1u);
   while ii < N {
      bibleN += bible;
      ii += 1u;
   }

   meow::time("lorem ipsum (100) in bible x260 (simple)",
              {|| simple(bibleN, str::slice(loremipsum, 0u, 100u))});
   meow::time("lorem ipsum (100) in bible x260 (BM)   ",
              {||    BM(bibleN, str::slice(loremipsum, 0u, 100u))});

   meow::time("lorem ipsum (400) in bible x260 (simple)",
              {|| simple(bibleN, str::slice(loremipsum, 0u, 400u))});
   meow::time("lorem ipsum (400) in bible x260 (BM)   ",
              {||    BM(bibleN, str::slice(loremipsum, 0u, 400u))});

*/



//   meow::time("lorem ipsum ^2 (simple)", {|| BM(loremipsum, loremipsum)} );
//   meow::time("lorem ipsum ^2 (BM)", {|| BM(loremipsum, loremipsum)} );


   // 2D range
   let (num_n, num_h) = (10u, 20u);
   let (mult_n, mult_h) = (100u, 1000u);
   let row1 = vec::to_mut(vec::from_elem(num_h, 1.0f));
   let result = vec::to_mut(vec::from_elem(num_n, row1));
   let mut nn = 0u;
   while nn < num_n {
      let mut hh = 0u;
      while hh < num_h {
         // save a grid of ratio of time for BM / simple search
         result[nn][hh] = compareHN(1u+hh*mult_h, 1u+nn*mult_n);

         hh += 1u;
      }
      nn += 1u;
   }

   let matlab_data_2d = fn@(data: [mut [mut float]]) -> (str,str,str) {
      // captured:
      //let mult_h, mult_n;

      let xxlim = vec::len(data);
      let yylim = vec::len(data[0]);

      let mut res = "";
      let mut xs = "";
      let mut ys = "";

      // Y
      let mut yy = 0u;
      while yy < yylim {
         if yy != 0u {
            res += "; ";
            xs += ", ";
         }

         // X
         let mut xx = 0u;
         while xx < xxlim {
            if xx != 0u {
               res += ", ";
               if yy == 0u {
                  ys += ", ";
               }
            }
            
            // data
            res += #fmt("%f", data[xx][yy]);

            if yy == 0u {
               ys += #fmt("%u", 1u + mult_n * xx);
            }
            xx += 1u;
         }

         xs += #fmt("%u", 1u + mult_h * yy);
         yy += 1u;
      }

      ret (xs, ys, res);
   };

   io::println("######################################################");
   io::println("### paste into Octave or Matlab... ###################");

   // output to octave/matlab
   let (xs, ys, ratio) = matlab_data_2d(result);
   io::println("ratio     = [" + ratio + "];");
   io::println("haystacks = [" + ys + "];");
   io::println("needles   = [" + xs + "];");

   io::println("contourf(haystacks, needles, ratio, [0, 1, 2, 3]);");
   io::println("xlabel('needle size');");
   io::println("ylabel('haystack size');");
   io::println("title('Boyer-Moore time / basic search time');");
   io::println("xlim([1 901]);");
   io::println("ylim([1 19001]);");
   io::println("colorbar;");
   io::println("");
   io::println("print('comparison.svg', '-dSVG');");

   io::println("######################################################");
}


