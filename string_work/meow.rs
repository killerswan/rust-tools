// benchmark string functions with very large samples
// such as the text of the bible, in lolcat
//
// WARNING: this is NOT Haskell's QuickCheck or Criterion,
// it is nearly the bare minimum I've done to feel like I'm actually
// performance testing

export time,
       sample_string,
       compare,
       compare_several,
       compare_sweep_strings,
       compare_sweep_strings_lim;

use std;

fn status (desc: str) {
   std::io::println("meow: " + desc);
}

fn status_two (desc: str, aa: float, bb: float) {
      status(desc + ": " + fmt_secs(aa) + ", " + fmt_secs(bb));
}

fn sample_string () -> str {
   let generator: std::rand::rng = std::rand::mk_rng();
   let random = generator.next();
   let sz = random / u32::max_value * 2048u32;
   ret generator.gen_str(sz as uint);
}

fn measure_time <XX> (action: fn&()->XX) -> float {
   let t0 = std::time::precise_time_s();
   action();
   let t1 = std::time::precise_time_s();
   ret t1 - t0;
} 

fn fmt_secs (secs: float) -> str {
   #fmt("%06.3f sec", secs)
}

// measure one function
fn time <XX> (desc: str, action: fn&()->XX) {
   status(desc + " " + fmt_secs(measure_time(action)));
}

// measure two functions
fn compare <XX> (desc: str,
                 actionA: fn&()->XX,
                 actionB: fn&()->XX) {

   status_two(desc, measure_time(actionA), measure_time(actionB));
}

// measure two, several iterations
fn compare_several <XX> (desc: str,
                         actionA: fn&()->XX,
                         actionB: fn&()->XX) {
   let tsA = [];
   let tsB = [];
   let jj = 0u;
   let nn = 7u;

   while jj < nn {
      vec::push(tsA, measure_time(actionA));
      vec::push(tsB, measure_time(actionB));
      jj += 1u;
   }
   
   status_two(desc + " (avg)", avg(tsA), avg(tsB)); 
}

fn avg (ts: [float]) -> float {
   ret vec::foldl(0f, ts, {|a,b| a+b})/(vec::len(ts) as float);
}

// measure two string functions,
// several lengths of iterations,
// on the same random data
fn compare_sweep_strings_lim <XX> (desc: str,
                           actionA: fn&(str)->XX,
                           actionB: fn&(str)->XX,
                           limit: uint) {

   let timesA = [];
   let timesB = [];

   let jj = 0u;
   let sz = 0u;

   let generator = std::rand::mk_rng();

   while jj < limit {
      let dataA = generator.gen_str(sz);
      let dataB = dataA;
   
      let runA = measure_time({|| actionA(dataA)});
      let runB = measure_time({|| actionB(dataB)});

      vec::push(timesA, runA);
      vec::push(timesB, runB);

      status_two(desc + #fmt(" of size: %8u", sz), runA, runB);

      if sz == 0u {
         sz += 1u;
      } else {
         sz *= 5u;
      }

      jj += 1u;
   }
}

fn compare_sweep_strings <XX> (desc: str,
                           actionA: fn&(str)->XX,
                           actionB: fn&(str)->XX) {
   compare_sweep_strings_lim(desc, actionA, actionB, 10u);
}


