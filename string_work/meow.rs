// benchmark string functions
// (originally, with a copy of the LOLCat bible!)
//
// WARNING: this is NOT Haskell's QuickCheck or Criterion :D

export time,                  // time one function call
       sample_string,         // provide a sample string < 2048 bytes
       compare,               // compare two functions
       compare_several,       // compare two functions (repeatedly)
       compare_sweep_strings, // compare two string functions on a given range of sizes
       compare_sweep_u8vecs;  // compare two [u8] funcs

use std;

fn status (desc: str) {
   std::io::println("meow: " + desc);
}

fn status_two (desc: str, aa: uint, bb: uint) {
      status(desc + ":\t" + fmt_ms(aa) + ", " + fmt_ms(bb));
}

fn sample_string () -> str {
   let generator: std::rand::rng = std::rand::mk_rng();
   let random = generator.next();
   let sz = random / u32::max_value * 2048u32;
   ret generator.gen_str(sz as uint);
}

fn measure_time <XX> (action: fn&()->XX) -> uint {
   let t0 = std::time::precise_time_ns();
   action();
   let t1 = std::time::precise_time_ns();
   ret t1 - t0;
} 

fn fmt_ms (nsecs: uint) -> str {
   #fmt("%06.3f ms", nsecs as float / 1e6f)
}

// measure one function
fn time <XX> (desc: str, action: fn&()->XX) {
   status(desc + " " + fmt_ms(measure_time(action)));
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
   let nn = 10u;

   while jj < nn {
      vec::push(tsA, measure_time(actionA));
      vec::push(tsB, measure_time(actionB));
      jj += 1u;
   }
   
   status_two(desc + " (avg)", avgu(tsA), avgu(tsB)); 
}

fn avgf (ts: [float]) -> float {
   ret vec::foldl(0f, ts, {|a,b| a+b})/(vec::len(ts) as float);
}

fn avgu (ts: [uint]) -> uint {
   ret vec::foldl(0u, ts, {|a,b| a+b})/(vec::len(ts) as uint);
}

// measure two string functions,
// several lengths of iterations,
// on the same random data
fn compare_sweep_strings <XX, YY> (
   desc: str,
   actionA: fn&(str)->XX,
   actionB: fn&(str)->YY,
   min_size: uint,
   max_size: uint
) {


   let iters = 5u;  // how many times to run at each size
   let steps = 10u;  // how many steps are in a sweep

   let size = min_size;

   let generator = std::rand::mk_rng();

   status(#fmt("Sweeping across strings of %u to %u (%u tests per step)...", 
               min_size,
               max_size,
               iters));

   // sweep through sizes
   while size <= max_size {
      let iter = 0u;
      let timesA = [];
      let timesB = [];

      while iter < iters {
         let dataA = generator.gen_str(size);
         let dataB = dataA;
      
         let runA = measure_time({|| actionA(dataA)});
         let runB = measure_time({|| actionB(dataB)});

         vec::push(timesA, runA);
         vec::push(timesB, runB);


         iter += 1u;
      }
      
      status_two(desc + #fmt(" (%u)", size), avgu(timesA), avgu(timesB));

      size += (max_size - min_size) / steps;
   } }

// measure two [u8] functions,
// several lengths of iterations,
// on the same random data
fn compare_sweep_u8vecs <XX, YY> (
   desc: str,
   actionA: fn&([u8])->XX,
   actionB: fn&([u8])->YY,
   min_size: uint,
   max_size: uint
) {


   let iters = 5u;  // how many times to run at each size
   let steps = 10u;  // how many steps are in a sweep

   let size = min_size;

   let generator = std::rand::mk_rng();

   status(#fmt("Sweeping across [u8] of %u to %u (%u tests per step)...", 
               min_size,
               max_size,
               iters));

   // sweep through sizes
   while size <= max_size {
      let iter = 0u;
      let timesA = [];
      let timesB = [];

      while iter < iters {
         let dataA = generator.gen_bytes(size);
         let dataB = dataA;
      
         let runA = measure_time({|| actionA(dataA)});
         let runB = measure_time({|| actionB(dataB)});

         vec::push(timesA, runA);
         vec::push(timesB, runB);


         iter += 1u;
      }
      
      status_two(desc + #fmt(" (%u)", size), avgu(timesA), avgu(timesB));

      size += (max_size - min_size) / steps;
   }
}


