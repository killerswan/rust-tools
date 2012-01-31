use std;

fn a () {
   std::io::println("a")
}

mod up {
   fn a () {
      std::io::println("A")
   }
}

fn b () {
   std::io::println("b")
}

mod up {
   fn b () {
      std::io::println("B")
   }
}


