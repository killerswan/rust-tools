// Kevin Cantu
// additional vector functions

use std;

fn windowed <copy TT> (nn: uint, xx: [TT]) -> [[TT]] {
   let ww = [];

   if nn < 1u {
      fail "windowed: n should be >= 1";
   }

   vec::iteri (xx, {|ii, _x|
      let len = vec::len(xx);

      if ii+nn <= len {
         let w = vec::slice ( xx, ii, ii+nn );
         vec::push (ww, w);
      }
   });

   ret ww;
}

#[test]
fn windowed_test_3of6 () {
   assert [[1u,2u,3u],[2u,3u,4u],[3u,4u,5u],[4u,5u,6u]]
          == windowed (3u, [1u,2u,3u,4u,5u,6u]);
}

#[test]
fn windowed_test_4of6 () {
   assert [[1u,2u,3u,4u],[2u,3u,4u,5u],[3u,4u,5u,6u]]
          == windowed (4u, [1u,2u,3u,4u,5u,6u]);
}

#[test]
#[should_fail]
fn windowed_test_0of6 () {
   let _x = windowed (0u, [1u,2u,3u,4u,5u,6u]);
}

#[test]
fn windowed_test_7of6 () {
   assert [] == windowed (7u, [1u,2u,3u,4u,5u,6u]);
}


