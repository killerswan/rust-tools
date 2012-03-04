use std;

fn find_str(haystack: str, needle: str) -> option<uint> {
    find_str_between(haystack, needle, 0u, str::len(haystack))
}

fn find_str_OLD(haystack: str, needle: str) -> option<uint> {
    find_str_between_OLD(haystack, needle, 0u, str::len(haystack))
}

fn find_str_between(haystack: str, needle: str, start: uint, end:uint)
  -> option<uint> {
    // THESE ARE BONE HEADED?
    assert end <= str::len(haystack);
    let needle_len = str::len(needle);
    if needle_len == 0u { ret some(start); }
    if needle_len > end { ret none; }

    let found = findn_str_between(haystack, needle, 1u, start, end);
    alt vec::len(found) {
        0u  { ret option::none; }
        _nn { ret option::some(found[0u]); }
    }
}

// PENDING: this replacement isn't exact...
// FIXME: DELETE
fn find_str_between_OLD(haystack: str, needle: str, start: uint, end:uint)
  -> option<uint> {

    assert end <= str::len(haystack);
    let needle_len = str::len(needle);
    if needle_len == 0u { ret some(start); }
    if needle_len > end { ret none; }

    let i = start, e = end - needle_len;
    while i <= e {
        if match_at(haystack, needle, i) { ret some(i); }
        i += 1u;
    }
    ret none;
}

// Function: findn_str
//
// Returns up to `nn` byte positions of matched substrings
fn findn_str(haystack: str, needle: str, nn: uint) -> [uint] {
    findn_str_between(haystack, needle, nn, 0u, str::len(haystack))
}

// Function: findn_str_between
//
// Returns up to `nn` byte positions of matched substrings
// between `start` and `end`
fn findn_str_between (haystack: str, needle: str,
                      nn: uint,
                      start: uint, end: uint) -> [uint] {
   ret boyer_moore_search(haystack, needle, nn, start, end);
}

// Returns up to `nn` byte positions of matched substrings
// between `start` and `end`
// (using Boyer-Moore)
fn boyer_moore_search (haystack: str, needle: str,
                      nn: uint,
                      start: uint, end: uint) -> [uint] {
    let results = [];

    let nlen = str::len(needle);
    let hlen = str::len(haystack);


    // empty needle
    if nlen == 0u {
        vec::push(results, start);
        ret results;
    }

    // empty haystack
    if hlen == 0u {
        ret results;
    }

    // needle too large
    if hlen < nlen {
        ret results;
    }

    // generate the tables
    let ct = boyer_moore_char_table(needle);
    let pt = boyer_moore_prefix_table(needle);

    log(error, "char_table:");
    for cc in needle { log(error, (str::from_bytes([cc]))); log(error, ct[cc]); }
    log(error, "prefix_table:");
    log(error, pt);

    // query both
    // based on position within the needle and character in haystack
    let getShift = fn@(pos: uint, ch: u8) -> uint {
        let matchedSoFar = nlen - 1u - pos;

        let charShift = ct[ch as uint] - matchedSoFar;
        let prefShift = pt[matchedSoFar];

        #error("<charShift: %u, prefShift: %u>", charShift, prefShift);

        if charShift > prefShift {
            ret charShift;
        } else {
            ret prefShift;
        }
    };

    // step up through the haystack
    let outerii = start;
    while outerii < end - nlen + 1u {

        // step back through needle
        let windowii = nlen;
        while 0u < windowii
              && (outerii + windowii - 1u) < hlen  // don't exceed haystack
        {
            windowii -= 1u;

            if needle[windowii] == haystack[outerii+windowii] {
                // still matching

                #error("<%c=%c, +0>", needle[windowii] as char, haystack[outerii+windowii] as char );

                if windowii == 0u {
                    // matched
                    vec::push(results, outerii);

                    if vec::len(results) >= nn {
                        ret results;
                    } else {
                        #error("<+%u>", nlen );
                        outerii += nlen;
                    }
                }
            } else {
                // not matching yet or was partial match
                //outerii += getShift(windowii, haystack[outerii+windowii]);
                let sh = getShift(windowii, haystack[outerii+windowii]);
                #error("<%c=%c, +%u>", needle[windowii] as char, haystack[outerii+windowii] as char, sh );
                outerii += sh;
                break;
            }
        }
    }

    ret results;
}

// compute the table used to choose a shift based on
// an unmatched character's possible position within the search string
// (a.k.a. the bad-character table)
fn boyer_moore_char_table (needle: str) -> [uint] {
    let len = str::len(needle);
    let mm  = vec::init_elt_mut(255u, len);

    let jj = len - 1u; // drop the last byte

    //assert 0u <= jj;
    //assert       jj < str::len(needle);

    // from last-1 to first
    while jj > 0u {
        jj -= 1u;

        let key = needle[jj] as uint;

        // if we haven't set it yet, set it now
        // (besides default)
        if mm[key] == len {
            mm[key] = len - 1u - jj;
        }
    }

    ret vec::from_mut(mm);
}

// compute the table used to choose a shift based on
// a partially matched suffix of the search string
// (a.k.a. the good-suffix table)
fn boyer_moore_prefix_table (needle_str: str) -> [uint] {
    let needle = str::bytes(needle_str);

    let len   = vec::len(needle);
    //assert 0u < len;

    // initialize len chars to len
    let mm  = vec::init_elt_mut(len, len);

    // step to larger suffixes
    let sii = 0u;
    while sii < len {

        // tail of the needle we seek
        let suffix      = vec::slice(needle, len - sii,      len);
        let suffix_plus = vec::slice(needle, len - sii - 1u, len);
        let slen = vec::len(suffix);

        // step to smaller prefixes
        let pii = len - 1u;
        while pii > 0u {

            // a prefix of the needle
            let prefix = vec::slice(needle, 0u, pii);
            let plen = vec::len(prefix);

            // if suffix fully matched, or
            // prefix is bigger than suffix: only tail matched
            // (which we might jump to)
            if
                (plen <= slen
                 && vec::ends_with(suffix, prefix))
            ||
                (slen < plen
                 && vec::ends_with(prefix, suffix)
                 && !vec::ends_with(prefix, suffix_plus))
            {
                // if we haven't set it yet, set it now
                // (besides default)
                if mm[sii] == len {
                    mm[sii] = len-pii;
                }
            }

            pii -= 1u;
        }

        // if it hasn't been set, there was no matching prefix,
        // so set it now
        if mm[sii] == len {
            mm[sii] = len-pii;
        }

        sii += 1u;
    }

    ret vec::from_mut(mm);
}

/*
Function: contains

Returns true if one string contains another

Parameters:

haystack - The string to look in
needle - The string to look for
*/
fn contains(haystack: str, needle: str) -> bool {
    //option::is_some(find_str_OLD(haystack, needle))
      // PENDING
    option::is_some(find_str_between_OLD(haystack, needle, 0u, str::len(haystack)))
}

