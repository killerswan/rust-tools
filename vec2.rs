// Kevin Cantu
// additional vector functions

use std;

fn windowed <copy TT> (nn: uint, xx: [TT]) -> [[TT]] {
   let ww = [];

   assert 1u <= nn;

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

fn splitAt <copy TT> (nn: uint, xx: [TT]) -> ([TT],[TT]) {
   assert 0u <= nn;
   assert nn <= vec::len(xx);
   (vec::slice(xx,0u,nn), vec::slice(xx,nn,vec::len(xx)))
}

#[test]
fn splitAt_test_a () {
   let temp: ([u8],[u8]) = splitAt(0u, []);
   assert ([],[]) == temp;
}

#[test]
fn splitAt_test_b () {
   assert ([],[1u]) == splitAt(0u, [1u]);
}

#[test]
fn splitAt_test_c () {
   assert ([1u],[]) == splitAt(1u, [1u]);
}

#[test]
#[should_fail]
fn splitAt_test_d () {
   let temp: ([u8],[u8]) = splitAt(7u, []);
   assert ([],[]) == temp;
}


fn take <copy TT> (nn: uint, xx: [TT]) -> [TT] {
   assert nn < vec::len(xx);
   ret vec::slice (xx, 0u, nn);
}

#[test]
fn take_test_a () {
   assert [] == take(0u, [8u,9u]);
}

#[test]
#[should_fail]
fn take_test_b () {
   let _xyz = take(6u, [8u,9u]);
}

// insert a str between elements of [str]
// then concatenate them all
fn intercalate (sep: str, xs: [str]) -> str {
   check vec::is_not_empty(xs);

   ret vec::foldl ( vec::head(xs),
                    vec::tail(xs),
                    {|acc,s| acc + sep + s});
} 


#[test]
fn intercalate_test_a () {
   let vv   =  ["abc","def","ghi","jkl","mno"];
   assert "abcXXdefXXghiXXjklXXmno" == intercalate("XX", vv);
}


// given a function to stringify an element
// make a pretty string of a vector
fn show <copy TT> ( vv: [TT],
                    showTT: block(TT)->str ) -> str {

   let strings = vec::map (vv, showTT);


   ret "[" + intercalate(", ", strings) + "]";
}

#[test]
fn show_test_a () {
   assert "[3, 56, 62, 2, 4]"
         == show ([3u,56u,62u,2u,4u], {|x| #fmt("%u", x)});
}

#[test]
fn show_test_b () {
   let vv   =  [[ "abc",  "def",  "ghi" ],[ "jkl",  "mno" ]];
   let ref  = "[[\"abc\", \"def\", \"ghi\"], [\"jkl\", \"mno\"]]";

   // this is a bit fragile, but works ok for now...
   let test = show (vv, {|xs|
                 show (xs, {|x| "\"" + x + "\"" })
              });

   assert ref == test;
}

   



