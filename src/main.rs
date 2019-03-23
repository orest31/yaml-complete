// usage `. <(yaml-complete <filename>)`

extern crate dirs;

use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::path::MAIN_SEPARATOR;
use std::process::Command;

use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        return Ok(());
    }

    let home_path = dirs::home_dir();
    if home_path.is_none() {
        return Err(Error::from(ErrorKind::NotFound));
    }

    let file_path = format!("{}{}.yaml-complete{}{}.yml",
                            home_path.unwrap().display(),
                            MAIN_SEPARATOR,
                            MAIN_SEPARATOR,
                            args[1]);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let cfg: Vec<Yaml> = YamlLoader::load_from_str(&contents).unwrap();

    if arg_len == 2 {
        for item in cfg {
            print_values(&item, &|_x| true);
        }
        return Ok(());
    }

    for item in &cfg {
        nested_value(item, &args[2..]);
    }

    Ok(())
}

fn nested_value(doc: &Yaml, args: &[String]) {
    if args.len() == 0 {
        print_values(doc, &|_x| true);
        return;
    }

    match *doc {
        Yaml::Hash(ref h) => {
            let arg = args[0].as_str();

            for (key, val) in h {
                if key.as_str().unwrap() == arg {
                    return nested_value(val, &args[1..]);
                }
            }
            if args.len() == 1 {
                print_values(doc, &|item| item.starts_with(arg));
            }
        }
        Yaml::String(ref _v) => {
            print_values(doc, &|_x| true);
        }
        _ => {}
    }
}

fn print_values(doc: &Yaml, filter: &Fn(&str) -> bool) {
    match *doc {
        Yaml::Hash(ref h) => {
            for (k, _v) in h {
                let val: &str = k.as_str().unwrap();
                if filter(val) {
                    println!("{}", val)
                }
            }
        }
        Yaml::Array(ref v) => {
            for list_item in v {
                let val: &str = list_item.as_str().unwrap();
                if filter(val) {
                    println!("{}", val);
                }
            }
        }
        Yaml::String(ref v) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(v)
                .output()
                .expect("failed to execute process");
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        _ => {}
    }
}
