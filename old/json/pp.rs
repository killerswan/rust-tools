#!/usr/bin/env rustx

use std;
import std::json;
import std::json::Json;
import std::json::Error;

fn main (args: ~[~str]) {
   if vec::len(args) != 2 { io::println(~"Usage: ./pp.rs JSON-FILE"); return; }

   let result_data: Result<~str, ~str> = io::read_whole_file_str(&path::Path(args[1]));
   let data = result::unwrap(result_data);

   let result_json: Result<Json, Error> = json::from_str(data);
   let json = result::get(result_json);

   io::println(json::to_str_pretty(json));
}

#[test]
fn test () {
   assert 2 + 2 == 4;
}

