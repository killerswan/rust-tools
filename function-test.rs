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
                                                                                                  
can also be    ~,@,&          &           &                                                       
used _as_                                                                                         
                                                                                                  
*/

// why do the types, fn(), fn~(), fn@(), and fn&() require parens?
// why isn't there a way to name a fn&?
// why isn't there a top level equivalent to fn for each of the others?


use std;


#[test]
fn testFunctionClosure () {

   let fruit = "banana";

   // top level fn syntax
   fn innerFunction1 () {
      let fruit = "orange";   // commenting this out leads to an error
                              // (does not make a fn& with closure)

      assert fruit == "orange";
   }

/* COMPILER ERROR
   // let fn syntax
   let innerFunction2 = native fn () {
      let fruit = "orange";
      assert fruit == "orange";
   };
*/

   innerFunction1 ();
//   innerFunction2 ();
   assert fruit == "banana";
}


#[test]
fn testSendfnClosure () {

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
fn testLambdaClosure () {

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
fn testBlockClosure () {

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


fn message () { (); }
fn usingFunction ( f : native fn() ) { f(); }
fn usingSendfn   ( f : fn~() ) { f(); }
fn usingLambda   ( f : fn@() ) { f(); }
fn usingBlock    ( f : fn& () ) { f(); }


#[test]
fn testFnTypes () {
   //let f1 = native fn     () { message (); }; // compiler error
   fn f1 () {
      message ();
   }
   let s1 = fn~ () { message (); };
   let l1 = fn@ () { message (); };

   usingFunction (f1);
   //usingFunction (s1);                        // compiler error
   //usingFunction (l1);                        // compiler error
   //usingFunction (fn& () { message ();});   // compiler error
   usingFunction ({|| message ();});

   usingSendfn (f1);
   usingSendfn (s1);
   //usingSendfn (l1);                          // compiler error
   //usingSendfn (fn& () { message ();});     // compiler error
   usingSendfn ({|| message ();});

   usingLambda (f1);
   //usingLambda (s1);                          // compiler error
   usingLambda (l1);
   //usingLambda (fn& () { message ();});     // compiler error
   usingLambda ({|| message ();});

   usingBlock (f1);
   usingBlock (s1);
   usingBlock (l1);
   usingBlock (fn& () { message ();});
   usingBlock ({|| message ();});
}


fn main () {
   std::io::println ("Running...");
}


