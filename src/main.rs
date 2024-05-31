use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let maps = (parse(APACHE), parse(DEBIAN), parse(GENTOO));
    println!("\nduplicates for Apache:");
    find_duplicates(&maps.0);
    println!("\nduplicates for Debian:");
    find_duplicates(&maps.1);
    println!("\nduplicates for Gentoo:");
    find_duplicates(&maps.2);
    
    let mut types = maps.0.keys().collect::<HashSet<_>>();
    types.extend(maps.1.keys());
    types.extend(maps.2.keys());

    let mut types = types.into_iter().collect::<Vec<_>>();
    types.sort_unstable();

    let empty = HashSet::new();
    for ty in &types {
        let mut exts = (
            maps.0.get(ty.as_str()).unwrap_or(&empty),
            maps.1.get(ty.as_str()).unwrap_or(&empty),
            maps.2.get(ty.as_str()).unwrap_or(&empty),
        );

        if exts.0 == exts.1 && exts.1 == exts.2 {
            continue;
        }

        let merged = exts.0 | exts.1;
        if &merged == exts.2 {
            continue;
        }

        if exts.2.is_superset(&merged) {
            println!("{} {:?} {:?} {:?}", ty, exts.0, exts.1, exts.2);
        }
    }
}

fn find_duplicates(map: &HashMap<String, HashSet<String>>) {
    let mut seen = HashMap::new();
    for (ty, exts) in map {
        for ext in exts {
            if let Some(prev) = seen.insert(ext, ty) {
                println!("duplicate extension {} for types {} and {}", ext, prev, ty);
            }
        }
    }
}

fn parse(fname: &str) -> HashMap<String, HashSet<String>> {
    BufReader::new(File::open(fname).unwrap())
        .lines()
        .filter_map(|ln| {
            let ln = ln.unwrap();
            if ln.is_empty() {
                return None;
            } else if ln.trim_start().starts_with('#') {
                return None;
            }

            let (mut ty, mut exts) = (None, HashSet::new());
            for s in ln.split_whitespace() {
                let s = s.to_owned();
                match ty {
                    Some(_) => {
                        exts.insert(s);
                    }
                    None => {
                        ty = Some(s);
                    }
                }
            }

            Some((ty.unwrap(), exts))
        })
        .collect()
}

const GENTOO: &str = "/etc/mime.types";
const APACHE: &str = "mime.types";
const DEBIAN: &str = "mime-support/mime.types";
