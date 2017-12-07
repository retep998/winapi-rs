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

fn check_feature_sorting<P: AsRef<Path>>(
    p: P,
    errors: &mut u32
) {
    let r_p = p.as_ref();
    let s_path = r_p.to_str().unwrap();
    let file_content = read_file(r_p);
    let mut features: Vec<(usize, String, String)> = Vec::new();
    for (pos, line) in file_content.lines().enumerate() {
        if !line.starts_with("#[cfg(feature") {
            continue
        }
        let feature_name = line.split(" ").last().unwrap().replace(";", "");
        features.push((pos + 1, feature_name.to_owned(), line.to_owned()));
    }
    if features.len() > 1 {
        for pos in 0..features.len() - 1 {
            if features[pos].1 > features[pos + 1].1 {
                writeln!(&mut io::stderr(), "[{}:{}] \"{}\" should be after \"{}\"",
                         s_path,
                         features[pos].0,
                         features[pos].2,
                         features[pos + 1].2).unwrap();
                *errors += 1;
            }
        }
    }
}

fn check_features_in_cargo_file(errors: &mut u32) {
    let file_content = read_file("Cargo.toml");
    let mut features: Vec<Vec<(usize, String, String)>> = Vec::new();
    let mut inside = false;
    for (pos, line) in file_content.lines().enumerate() {
        let line = line.trim();
        if line.starts_with("[features]") {
            inside = true;
            continue
        }
        if !inside {
            continue
        } else if line.starts_with("#") {
            if !features.is_empty() && !features.last().unwrap().is_empty() {
                features.push(Vec::new());
            }
            continue
        }
        if line.is_empty() {
            break
        }
        let feature_name = line.split(" ").next().unwrap().replace("\"", "");
        if features.is_empty() {
            features.push(Vec::new());
        }
        let len = features.len() - 1;
        features[len].push((pos + 1, feature_name.to_owned(), line.to_owned()));
    }
    if features.len() > 1 {
        for pos in 0..features.len() - 1 {
            for sub_pos in 0..features[pos].len() - 1 {
                if features[pos][sub_pos].1 > features[pos][sub_pos + 1].1 {
                    writeln!(&mut io::stderr(), "[{}:{}] \"{}\" should be after \"{}\"",
                             "Cargo.toml",
                             features[pos][sub_pos].0,
                             features[pos][sub_pos].2,
                             features[pos][sub_pos + 1].2).unwrap();
                    *errors += 1;
                }
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
        } else if path.to_str().unwrap().ends_with("mod.rs") {
            check_feature_sorting(path, errors);
        }
    }
}

#[test]
fn check_imports_sorting() {
    let mut errors = 0;
    read_dirs("src", &mut errors);
    check_features_in_cargo_file(&mut errors);
    assert!(errors == 0, "Not sorted feature(s) found");
}
