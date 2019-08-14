// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(p: P) -> String {
    let mut f = File::open(p).expect("read_file::open failed");
    let mut content =
        String::with_capacity(f.metadata().map(|m| m.len() as usize + 1).unwrap_or(0));
    f.read_to_string(&mut content)
        .expect("read_file::read_to_end failed");
    content
}

pub fn get_between_quotes(s: &str) -> &str {
    s.split('"').skip(1).next().unwrap_or("")
}
