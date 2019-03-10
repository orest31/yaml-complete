// usage `. <(yaml-complete <filename>)`

use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::path::MAIN_SEPARATOR;
use std::process::Command;

use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

extern crate dirs;

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
            print_values(&item);
        }
        return Ok(());
    }

    for item in &cfg {
        let val: Option<&Yaml> = nested_value(item, &args[2..]);
        if val.is_some() {
            print_values(val.unwrap());
        }
    }

    Ok(())
}

fn nested_value<'a>(doc: &'a Yaml, args: &[String]) -> Option<&'a Yaml> {
    if args.len() == 0 {
        return Some(doc);
    }

    match *doc {
        Yaml::Hash(ref h) => {
            let arg = args[0].as_str();

            for (key, val) in h {
                if key.as_str().unwrap() == arg {
                    return nested_value(val, &args[1..]);
                }
            }
        }
        Yaml::String(ref _v) => {
            return Some(doc);
        }
        _ => {
            return None;
        }
    }

    Some(doc)
}

fn print_values(doc: &Yaml) {
    match *doc {
        Yaml::Hash(ref h) => {
            for (k, _v) in h {
                println!("{}", k.as_str().unwrap_or_else(|| ""))
            }
        }
        Yaml::Array(ref v) => {
            for list_item in v {
                println!("{}", list_item.as_str().unwrap_or_else(|| ""))
            }
        }
        Yaml::String(ref v) => {
            let output = Command::new("sh")
                .arg("-c")
                .arg(v)
                .output()
                .expect("failed to execute process");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        _ => {}
    }
}
