// Copyright © 2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

#[cfg(windows)]
#[test]
fn check_imports() {
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fs::{File, read_dir};
    use std::io::{self, Read, Write};
    use std::path::Path;

    fn get_between_quotes(s: &str) -> &str {
        s.split('"').skip(1).next().unwrap_or("")
    }

    fn read_file<P: AsRef<Path>>(p: P) -> String {
        let mut f = File::open(p).expect("read_file::open failed");
        let mut content = String::new();
        f.read_to_string(&mut content).expect("read_file::read_to_end failed");
        content
    }

    fn check_if_in_build<P: Debug>(path: &P, include: &str, entries: &[String], errors: &mut u32) {
        for entry in entries {
            if &include == &entry {
                return
            }
        }
        io::stderr().write(format!("{:?}: include not found: \"{}\"\n",
                                   path,
                                   include).as_bytes()).expect("stderr::write failed");
        *errors += 1;
    }

    fn check_file_deps<P: AsRef<Path>>(p: P, files_deps: &mut HashMap<String, Vec<String>>,
                                       errors: &mut u32, level: u32) {
        let r_p = p.as_ref();
        let filename = if level < 2 {
            r_p.file_name().unwrap().to_str().unwrap().to_owned()
        } else {
            let values: Vec<String> = r_p.iter()
                                         .skip(1)
                                         .map(|x| x.to_str().unwrap().to_owned())
                                         .collect();
            values[values.len() - 2..].join("-")
        };
        {
            let entry = files_deps.get(&filename);
            if let Some(entry) = entry {
                let file_content = read_file(r_p);
                for line in file_content.lines() {
                    if line.starts_with("use ") {
                        let include: Vec<&str> = line.split("::").skip(1).collect();
                        if include.len() < 2 || include[0].starts_with('{') {
                            continue
                        }
                        let include = if include.len() > 2 {
                            include[..include.len() - 1].join("-").to_owned()
                        } else {
                            include[0].to_owned()
                        };
                        check_if_in_build(&r_p, &include, &entry, errors);
                    }
                }
            } else {
                return
            }
        }
        files_deps.remove(&filename);
    }

    fn read_dirs<P: AsRef<Path>>(dir: P, files_deps: &mut HashMap<String, Vec<String>>,
                                 errors: &mut u32, level: u32) {
        for entry in read_dir(dir).expect("read_dir failed") {
            let entry = entry.expect("entry failed");
            let path = entry.path();
            if path.is_dir() {
                read_dirs(path, files_deps, errors, level + 1);
            } else {
                check_file_deps(path, files_deps, errors, level);
            }
        }
    }

    let content = read_file("build.rs");
    let mut inside = false;
    let mut files_deps: HashMap<String, Vec<String>> = HashMap::new();

    for line in content.lines() {
        let line = line.trim_left();
        if !inside && line.starts_with("const DATA: ") {
            inside = true;
        } else if inside == true {
            let line = line.trim_left();
            if line.starts_with("//") {
                continue
            } else if !line.starts_with("(\"") {
                break
            }
            let parts: Vec<&str> = line.split("&[").collect();
            files_deps.insert(format!("{}.rs", get_between_quotes(parts[0])),
                              parts[1].split(',')
                                      .map(|x| get_between_quotes(x).to_owned()).collect());
        }
    }
    let mut errors = 0;
    read_dirs("src", &mut files_deps, &mut errors, 0);
    assert!(errors == 0, "Missing declaration(s) found");
    let keys: Vec<&String> = files_deps.keys().collect();
    if !keys.is_empty() {
        io::stderr().write(format!("file(s) not found: {:?}\n",
                                   keys).as_bytes()).expect("stderr::write failed");
        panic!();
    }
}
