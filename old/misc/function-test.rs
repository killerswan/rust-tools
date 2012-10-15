// Kevin Cantu
// basic Rust function behavior


/*
               native fn       fn~        fn@            fn&                                      
                                                                                                  
closures       none            unique/     boxed         stack/block                              
                               sendable                                                           
                                                                                                  
closure        n/a            immutable   immutable      mutable                                  
mutability                                                                                        
                                                                                                  
statmt def     yes            no          no             no          e.g., fn xyz () {...}        
                                                                                                  
let def        no             yes         yes            no          e.g., let xyz = fn@ () {...};
                                                                                                  
{||} def       yes            yes         yes            yes         e.g., abc({|| ...});         
                                                                                                  
can also be    fn~,fn@,fn&    fn          fn             fn                                       
used _as_                                                                                         
                                                                                                  
*/

// Why do the types native fn(), fn~(), fn@(), and fn&() require parens?
// Why isn't there a way to name a fn&?


use std;

fn message () { (); }



#[cfg(test)]
mod native {

   #[test]
   fn statement () {
      let fruit = "banana";

      fn innerFunction1 () {
         let fruit = "orange";   // commenting this out leads to an error
                                 // (does not make a fn& with closure)
         assert fruit == "orange";
      }

      innerFunction1 ();
      assert fruit == "banana";
   }

/*
   #[test]
   fn expression () {
      fail
      let innerFunction2 = native fn () {
         let fruit = "orange";
         assert fruit == "orange";
      };

      innerFunction2 ();
      assert fruit == "banana";
   }
*/


   fn call ( f : native fn() ) { f(); }

   #[test]
   fn with_native () {
      fn f1 () { message (); }
      call(f1);
   }

/*
   #[test]
   // compiler error
   fn with_sendfn () {
      let s1 = fn~ () { message (); };
      call (s1);
   }
*/

/*
   #[test]
   // compiler error
   fn with_lambda () {
      let l1 = fn@ () { message (); };
      call (l1);                        // compiler error
   }
*/

/*
   #[test]
   // compiler error
   fn with_block () {
      call (fn& () { message ();});   // compiler error
   }
*/

   #[test]
   fn with_literal () {
      call ({|| message ();});
   }
}

#[cfg(test)]
mod sendfn {
   #[test]
   fn expression () {
      let fruit = "banana";
      
      // let sendfn syntax
      let innerSendfn = fn~ () {
         assert fruit == "banana";

         // overwrite the value
         let fruit = "mango";
         assert fruit == "mango";
      };
      
      innerSendfn ();
      
      assert fruit == "banana";
   }   

   #[test]
   fn statement () {
   }

   fn call ( f : fn~() ) { f(); }

   #[test]
   fn with_native () {
      fn f1 () { message (); }
      call(f1);
   }

   #[test]
   fn with_sendfn () {
      let s1 = fn~ () { message (); };
      call (s1);
   }

/*
   #[test]
   fn with_lambda () {
      let l1 = fn@ () { message (); };
      call (l1);
   }

   #[test]
   fn with_block () {
      call (fn& () { message ();});
   }
*/

   #[test]
   fn with_literal () {
      call ({|| message ();});
   }
}



#[cfg(test)]
mod lambda {
   fn call   ( f : fn@() ) { f(); }

   #[test]
   fn expression () {

      let fruit = "banana";
      
      // let lambda syntax
      let innerLambda = fn@ () {
         assert fruit == "banana";

         // overwrite the value
         let fruit = "tangerine";
         assert fruit == "tangerine";
      };
      
      innerLambda ();
      
      assert fruit == "banana";
   }

   #[test]
   fn with_native () {
      fn f1 () { message (); }

      call (f1);
   }

/*
   #[test]
   fn with_sendfn () {
      let s1 = fn~ () { message (); };
      call (s1);                          // compiler error
   }
*/

   #[test]
   fn with_lambda () {
      let l1 = fn@ () { message (); };
      call (l1);
   }

/*
   #[test]
   fn with_block () {
      call (fn& () { message ();});     // compiler error
   }
*/

   #[test]
   fn with_literal () {
      call ({|| message ();});
   }
}

#[cfg(test)]
mod block {
   fn call    ( f : fn& () ) { f(); }

   #[test]
   fn expression () {

      let fruit = "banana";

      fn run1 ( b : fn& () ) {
         let i = 1u;
         while (i > 0u) { i = 0u;
            b ();
         }
      }
      
      // callee explicit fn& syntax
      // (Is there no way to define a fn&
      // before positioning it as an argument?)
      run1 ( fn& () {
         assert fruit == "banana";

         // overwrite the value
         fruit = "pineapple";
         assert fruit == "pineapple";
      });
      
      assert fruit == "pineapple";
   }

   #[test]
   fn with_native () {
      fn f1 () { message (); }
      call (f1);
   }

/*
   #[test]
   fn with_sendfn () {
      let s1 = fn~ () { message (); };
      //call (s1);
   }

   #[test]
   fn with_lambda () {
      let l1 = fn@ () { message (); };
      //call (l1);
   }
*/

   #[test]
   fn with_block () {
      call (fn& () { message ();});
   }

   #[test]
   fn with_literal () {
      call ({|| message ();});
   }

}

#[cfg(test)]
mod fn {
   fn call       ( f : fn () ) { f(); }

   #[test]
   fn with_native () {
      fn f1 () { message (); }
      call (f1);
   }

   #[test]
   fn with_sendfn () {
      let s1 = fn~ () { message (); };
      call (s1);
   }

   #[test]
   fn with_lambda () {
      let l1 = fn@ () { message (); };
      call (l1);
   }

   #[test]
   fn with_block () {
      call (fn& () { message ();});
   }

   #[test]
   fn with_literal () {
      call ({|| message ();});
   }
}


fn main () {
   std::io::println ("Running...");
}


