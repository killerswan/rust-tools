use std;
use bm;

fn run(result: option<uint>, haystack: str, needle: str, start: uint, end: uint) {
   assert result == bm::find_str_between    (haystack, needle, start, end);
   assert result == bm::find_str_between_OLD(haystack, needle, start, end);
}

//#[test]
fn AA() {
   run(option::none, "", "ok?", 0u, 0u);
   run(option::some(0u), "", "", 0u, 0u);
   run(option::some(0u), "hmm", "", 0u, 3u);
}

//#[test]
fn BB() {
   run(option::some(0u), "banana", "b", 0u, 6u);
   run(option::some(4u), "apple", "e",     0u, 5u);
   run(option::none, "apple", "xapple", 0u, 5u);
}

#[test]
fn CC() {
    let markdown =
"# Crate

## Function `b`

    fn b()

## Function `d`

    fn d()

# Module `a`

# Module `c`";


// ADDING A CHAR ON LINE 34 causes `# Module `a`` to be found OK.
// BUT CAUSES `# Module `c`` to not be found.
// THEN ADDING A SPACE ON LINE 36 causes `# Module `c`` to be found OK.

    assert str::is_utf8(str::bytes(markdown));

    std::io::println("KEVIN: should_write_modules_last called!");
    std::io::println("+++++++++++++++");
    std::io::println(markdown);
    std::io::println("+++++++++++++++");

    log(error, bm::findn_str(markdown, "#", 20u));
    log(error, bm::findn_str(markdown, "## ", 20u));
    log(error, bm::findn_str(markdown, "# Module `a", 20u));
    log(error, bm::findn_str(markdown, "# Module `a`", 20u)); // NULL????
    std::io::println(#fmt("len of query: %u", str::len("# Module `a`")));
    log(error, bm::findn_str(markdown, "# Module `c", 20u));
    log(error, bm::findn_str(markdown, "# Module `c`", 20u)); // NULL????

    let idx_a = option::get(bm::find_str(markdown, "# Module `a`")); // ERROR
    let idx_b = option::get(bm::find_str(markdown, "## Function `b`"));
    let idx_c = option::get(bm::find_str(markdown, "# Module `c`")); // ERROR
    let idx_d = option::get(bm::find_str(markdown, "## Function `d`"));

    assert idx_b < idx_d;
    assert idx_d < idx_a;
    assert idx_a < idx_c;

    assert option::none == bm::find_str(markdown, "## Function `z`");
}


