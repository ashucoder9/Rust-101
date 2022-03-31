/* Ferris is the official mascot of Rust-lang
Put the following in your Cargo.toml:
    [dependencies]
    ferris-says = "0.2"
    
Now run "cargo run build" command
*/

extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello Rustaceans, This is Ferris");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

/* OUTPUT EXPECTED

__________________
< Hello Rustaceans, This is Ferris >
 ------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
          
*/
