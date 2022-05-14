use std::{fs, io};
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::format;
use std::ops::Index;

use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use yaml_rust::yaml::Array;

fn main() {
    let fileData = fs::read_to_string("indefinitebans.yml").expect("Unable to read this file! Perhaps it's corrupt or the file was not found.");
    let yaml = &YamlLoader::load_from_str(&fileData).expect("This file couldn't be loaded as a YAML")[0];
    let parentHash = yaml.as_hash().expect("Unable to load everything into a hash");
    let mut map: BTreeMap<usize, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let mut index = 0;
    for i in 0..parentHash.keys().len()
    {
        let k = parentHash.iter().skip(i).next().unwrap().0;
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
        let hash = parentHash[k].as_hash().expect("Couldn't load key as hash");
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
}

fn add_one(num: &mut i32) {
    *num += 1;
}
