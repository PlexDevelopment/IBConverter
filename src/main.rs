use std::{env, fs, io};
use std::collections::{BTreeMap};
use std::io::{Read};
use std::path::Path;

use yaml_rust::{Yaml, YamlLoader};

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    println!("Welcome to IBConverter. This program converts TotalFreedomMod indefinite bans to Plex's format.");
    println!("Path to TotalFreedomMod indefinite bans YAML file:");
    // Path::new(".").read_dir().unwrap().for_each(|f| println!("{:?}", f.unwrap().file_name()));
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Unable to parse this path.");
    if !path.starts_with("/") {
        path.insert_str(0, "./");
    }
    path = path.replace("\n", "").replace("\\n", "").replace("\r", "");
    while !Path::new(&path).exists() {
        println!("Unable to find {:?}, please provide a valid path.", &path);
        path = "".to_string();
        io::stdin().read_line(&mut path).expect("Unable to parse this path.");
        path = path.replace("\n", "").replace("\\n", "").replace("\r", "");
    }
    let file_data = fs::read_to_string(&path).expect("Unable to read this file! Perhaps it's corrupt or the file was not found.");
    let yaml = &YamlLoader::load_from_str(&file_data).expect("This file couldn't be loaded as a YAML")[0];
    let parent_hash = yaml.as_hash().expect("Unable to load everything into a hash");
    let mut map: BTreeMap<usize, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    for i in 0..parent_hash.keys().len()
    {
        let k = parent_hash.iter().skip(i).next().unwrap().0;
        println!("Key: {:?}", k);
        let mut data: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut names: Vec<String> = Vec::new();
        if k.as_str().is_some() {
            names.push(k.as_str().unwrap().to_string());
            data.insert("users".to_string(), names);
        } else {
            let str = String::from(k.to_owned().as_i64().clone().unwrap().to_string());
            names.push(str);
            data.insert("users".to_string(), names);
        }
        let hash = parent_hash[k].as_hash().expect("Couldn't load key as hash");
        if hash.contains_key(&Yaml::String("uuid".to_string())) {
            let mut uuids: Vec<String> = Vec::new();
            uuids.push(hash.get(&Yaml::String("uuid".to_string())).unwrap().to_owned().into_string().unwrap());
            data.insert("uuids".to_string(), uuids);
        }
        if hash.contains_key(&Yaml::String("ips".to_string())) {
            if hash.get(&Yaml::String("ips".to_string())).unwrap().is_array() {
                let mut ips: Vec<String> = Vec::new();
                for x in hash.get(&Yaml::String("ips".to_string())).unwrap().as_vec().unwrap() {
                    ips.push(x.to_owned().into_string().unwrap());
                }
                data.insert("ips".to_string(), ips);
            }
        }
        map.insert(i, data);
        println!("Inserted {}", i);
    }

    let s = serde_yaml::to_string(&map).expect("Unable to convert to string");
    println!("{:?}", s);
    println!("{:?}", &s[0..4]);
    fs::write("./indefbans.yml", &s[4..]).expect("Unable to write to file");

    let mut stdin = io::stdin();
    println!("Press any key to continue...");
    let _ = stdin.read(&mut [0u8]).unwrap();
}