use std;
fn main () {
 let data = "abracadabra";
 let _x = str::windowed(3u, data);
 let _y = str::lines_iter(data, {|x| std::io::println(x)});
 let _z = str::words_iter(data, {|x| std::io::println(x)});
 let _a = str::all(data, char::is_uppercase);
 let _b = str::any(data, char::is_uppercase);
 let _c = str::map(data, char::to_upper);
}
