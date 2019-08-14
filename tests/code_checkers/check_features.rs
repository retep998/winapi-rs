// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use std::fs::read_dir;
use std::io::{self, Write};
use std::path::Path;

use utils::{get_between_quotes, read_file};

fn get_libs() -> Vec<String> {
    let content = read_file("build.rs");
    let mut inside = false;
    let mut files_deps = Vec::new();
    for line in content.lines() {
        let line = line.trim_left();
        if !inside && line.starts_with("const DATA: ") {
            inside = true;
        } else if inside == true {
            let line = line.trim_left();
            if line.starts_with("//") {
                continue;
            } else if !line.starts_with("(\"") {
                break;
            }
            let parts: Vec<&str> = line.split("&[").collect();
            files_deps.push(get_between_quotes(parts[0]).to_owned());
        }
    }
    files_deps
}

fn check_feature_sorting<P: AsRef<Path>>(p: P, errors: &mut u32) -> Vec<String> {
    let r_p = p.as_ref();
    let s_path = r_p.to_str().unwrap();
    let file_content = read_file(r_p);
    let mut features: Vec<(usize, String, String)> = Vec::new();
    for (pos, line) in file_content.lines().enumerate() {
        if !line.starts_with("#[cfg(feature") {
            continue;
        }
        let feature_name = line.split(" ").last().unwrap().replace(";", "");
        features.push((pos + 1, feature_name.to_owned(), line.to_owned()));
    }
    if features.len() > 1 {
        for pos in 0..features.len() - 1 {
            if features[pos].1 > features[pos + 1].1 {
                writeln!(
                    &mut io::stderr(),
                    "[{}:{}] \"{}\" should be after \"{}\"",
                    s_path,
                    features[pos].0,
                    features[pos].2,
                    features[pos + 1].2
                )
                .unwrap();
                *errors += 1;
            }
        }
    }
    features.into_iter().map(|e| e.1).collect()
}

fn check_features_in_cargo_file(errors: &mut u32) -> Vec<String> {
    let file_content = read_file("Cargo.toml");
    let mut features: Vec<Vec<(usize, String, String)>> = Vec::new();
    let mut inside = false;
    for (pos, line) in file_content.lines().enumerate() {
        let line = line.trim();
        if line.starts_with("[features]") {
            inside = true;
            continue;
        }
        if !inside {
            continue;
        } else if line.starts_with("#") {
            if !features.is_empty() && !features.last().unwrap().is_empty() {
                features.push(Vec::new());
            }
            continue;
        }
        if line.is_empty() {
            break;
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
                    writeln!(
                        &mut io::stderr(),
                        "[{}:{}] \"{}\" should be after \"{}\"",
                        "Cargo.toml",
                        features[pos][sub_pos].0,
                        features[pos][sub_pos].2,
                        features[pos][sub_pos + 1].2
                    )
                    .unwrap();
                    *errors += 1;
                }
            }
        }
    }
    features
        .into_iter()
        .flat_map(|e| e.into_iter().map(|el| el.1))
        .collect()
}

fn check_missing_features_in_cargo_file(
    build_features: &[String],
    cargo_features: &[String],
    errors: &mut u32,
) {
    const FEATURES_TO_IGNORE: &'static [&'static str] =
        &["debug", "everything", "impl-debug", "impl-default", "std"];
    let mut it1 = 0;
    let mut it2 = 0;

    while it1 < build_features.len() && it2 < cargo_features.len() {
        if build_features[it1] == cargo_features[it2] {
            it1 += 1;
            it2 += 1;
            continue;
        } else if FEATURES_TO_IGNORE
            .iter()
            .any(|e| *e == &cargo_features[it2])
        {
            it2 += 1;
            continue;
        } else if build_features[it1] < cargo_features[it2] {
            writeln!(
                &mut io::stderr(),
                "Missing feature \"{}\" in `Cargo.toml`",
                build_features[it1]
            )
            .unwrap();
            *errors += 1;
            while it1 < build_features.len() && build_features[it1] < cargo_features[it2] {
                it1 += 1;
            }
        } else {
            writeln!(
                &mut io::stderr(),
                "Extra feature in `Cargo.toml`: \"{}\"",
                cargo_features[it2]
            )
            .unwrap();
            *errors += 1;
            while it2 < cargo_features.len() && build_features[it1] > cargo_features[it2] {
                it2 += 1;
            }
        }
    }
}

fn read_dirs<P: AsRef<Path>>(dir: P, errors: &mut u32) {
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
    let mut cargo_features = check_features_in_cargo_file(&mut errors);
    if errors == 0 {
        // No need to check for missing features in here since we have sorting issues. Outcome would
        // be chaotic.
        cargo_features.sort();
        let mut build_features: Vec<String> = get_libs();
        build_features.sort();
        check_missing_features_in_cargo_file(&build_features, &cargo_features, &mut errors);
    }
    assert!(errors == 0, "Not sorted feature(s) found");
}
