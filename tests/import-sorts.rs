// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::fs::{File, read_dir};
use std::io::{self, Read, Write};
use std::path::Path;

fn read_file<P: AsRef<Path>>(p: P) -> String {
    let mut f = File::open(p).expect("read_file::open failed");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("read_file::read_to_end failed");
    content
}

fn check_inner_imports(
    filename: &str,
    line_pos: usize,
    line: &str,
    imports: &[String],
    errors: &mut u32
) {
    if imports.len() > 1 {
        for pos in 0..imports.len() - 1 {
            if imports[pos] == "self" {
                continue
            }
            if imports[pos] > imports[pos + 1] {
                writeln!(&mut io::stderr(), "[{}:{}] In \"{}\": \"{}\" should be after \"{}\"",
                         filename,
                         line_pos,
                         line,
                         imports[pos],
                         imports[pos + 1]).unwrap();
                *errors += 1;
            }
        }
    }
}

fn check_import_sorting<P: AsRef<Path>>(
    p: P,
    errors: &mut u32
) {
    let r_p = p.as_ref();
    let s_path = r_p.to_str().unwrap();
    let file_content = read_file(r_p);
    let mut imports: Vec<(usize, Vec<String>)> = Vec::new();
    let mut current_import: Option<Vec<String>> = None;
    for (pos, line) in file_content.lines().enumerate() {
        if !line.starts_with("use ") && current_import.is_none() {
            continue
        }
        if current_import.is_some() {
            let line = line.trim();
            if line.ends_with("};") {
                if let Some(ref current_import) = current_import {
                    {
                        let last = imports.last().unwrap();
                        check_inner_imports(s_path,
                                            last.0,
                                            &last.1.join("::"),
                                            &current_import, errors);
                    }
                    let len = imports.len() - 1;
                    imports[len].1.push(format!("{{{}}};", current_import.join(", ")));
                }
                current_import = None;
            } else if let Some(ref mut current_import) = current_import {
                current_import.push(line.replace(",", "").trim().to_owned());
            }
        } else {
            let new_entry: Vec<String> = line.split("use ")
                                             .skip(1)
                                             .next()
                                             .unwrap()
                                             .split("::")
                                             .map(|s| s.to_owned())
                                             .collect();
            if let Some(last) = new_entry.last() {
                if last.ends_with("};") {
                    check_inner_imports(s_path,
                                        pos + 1,
                                        &new_entry.join("::"),
                                        &last.replace("{", "")
                                             .replace("}", "")
                                             .replace(";", "")
                                             .trim()
                                             .split(",")
                                             .map(|s| s.trim().to_owned())
                                             .collect::<Vec<String>>(), errors);
                } else {
                    if last.split(',').count() == 0 {
                        current_import = Some(Vec::new());
                    }
                }
            }
            imports.push((pos + 1, new_entry));
        }
    }
    if imports.len() > 1 {
        for pos in 0..imports.len() - 1 {
            let mut i = 0;
            while i < imports[pos].1.len() - 1 && i < imports[pos + 1].1.len() - 1 &&
                  imports[pos].1[i] == imports[pos + 1].1[i] {
                i += 1;
            }
            if i >= imports[pos].1.len() - 1 {
                continue
            }
            if i >= imports[pos + 1].1.len() - 1 || imports[pos].1[i] > imports[pos + 1].1[i] {
                writeln!(&mut io::stderr(), "[{}:{}] \"use {}\" should be after \"use {}\"",
                         s_path,
                         imports[pos].1.join("::"),
                         imports[pos].0,
                         imports[pos + 1].1.join("::")).unwrap();
                *errors += 1;
            }
        }
    }
}

fn read_dirs<P: AsRef<Path>>(
    dir: P,
    errors: &mut u32
) {
    for entry in read_dir(dir).expect("read_dir failed") {
        let entry = entry.expect("entry failed");
        let path = entry.path();
        if path.is_dir() {
            read_dirs(path, errors);
        } else {
            check_import_sorting(path, errors);
        }
    }
}

#[test]
fn check_imports_sorting() {
    let mut errors = 0;
    read_dirs("src", &mut errors);
    assert!(errors == 0, "Not sorted import(s) found");
}
