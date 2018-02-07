// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::fs::read_dir;
use std::io::{self, Write};
use std::path::Path;

mod utils;

fn check_if_files_in_mod(
    files: &[String],
    mod_file: &Path,
    errors: &mut u32,
) {
    let content = utils::read_file(mod_file);
    let mut imports = HashMap::new();
    content.split('\n')
           .filter_map(|s| {
               let x: Vec<&str> = s.split("mod ").collect();
               if x.len() < 2 {
                   None
               } else {
                   // We assume that only one mod import is present on a line.
                   x[1].split(';').next()
               }
           })
           .for_each(|x| {
               imports.insert(x.to_owned(), false);
           });
    for file in files {
        if let Some(import) = imports.get_mut(file) {
            *import = true;
        } else {
            writeln!(&mut io::stderr(),
                     "\"{}\" isn't imported in \"{}\"",
                     file,
                     mod_file.display()).unwrap();
            *errors += 1;
        }
    }

    // Just because we want to have checks without compilation!
    for (import, found) in &imports {
        if *found == false {
            writeln!(&mut io::stderr(),
                     "module \"{}\" is imported in \"{}\" but doesn't exist",
                     import,
                     mod_file.display()).unwrap();
            *errors += 1;
        }
    }
}

fn read_dirs<P: AsRef<Path>>(
    dir: P,
    errors: &mut u32,
) {
    let mut files = Vec::new();
    let mut mod_file = None;

    for entry in read_dir(dir).expect("read_dir failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();
        if path.is_dir() {
            read_dirs(path, errors);
            files.push(entry.file_name().into_string().unwrap());
        } else {
            let file_name = entry.file_name().into_string().unwrap();
            if file_name != "mod.rs" {
                files.push(file_name.replace(".rs", ""));
            } else {
                mod_file = Some(path);
            }
        }
    }
    if !files.is_empty() && mod_file.is_some() {
        check_if_files_in_mod(&files, &mod_file.unwrap(), errors);
    }
}

#[test]
fn check_all_files_are_used() {
    let mut errors = 0;
    read_dirs("src", &mut errors);
    assert!(errors == 0, "Not sorted feature(s) found");
}
