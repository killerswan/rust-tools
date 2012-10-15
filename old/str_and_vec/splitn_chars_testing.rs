use std;

    #[test]
    fn test_splitn_chars_iter() {
        let data = "\nMary had a little lamb\nLittle lamb\n";

        let ii = 0;

        vec::iter(str::splitn(data, ' ' as u8, 2u), {|xx|
            alt ii {
              0 { assert "\nMary" == xx; }
              1 { assert "had"    == xx; }
              2 { assert "a little lamb\nLittle lamb\n"    == xx; }
              _ { () }
            }
            ii += 1;
        });
    }

