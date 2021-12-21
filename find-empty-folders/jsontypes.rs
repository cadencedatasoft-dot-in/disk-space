
use std::ffi::OsStr;
use std::{fs, collections::HashMap};
use serde::Serialize;
use serde::Deserialize;
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Jsontypes{
    pub ft: HashMap<String, u64>,
    pub known_types: HashMap<String, (Vec<String>,u64)>,
    pub known_cat: HashMap<String, u64>,
}

impl Jsontypes {
    pub fn new(path: &OsStr) -> Self{
            match Jsontypes::read_exts(path){
                None => {
                    Jsontypes{
                        ft: HashMap::new(),
                        known_types: HashMap::new(),
                        known_cat: HashMap::new()
                    }
                },
                Some(x) => {
                    Jsontypes{
                        ft: x.0,
                        known_types: x.1,
                        known_cat: x.2
                    }
                },
            }
        }

    pub fn read_exts(path: &OsStr) -> Option<(HashMap<String, u64>, HashMap<String, (Vec<String>, u64)>, HashMap<String, u64>)> {
        let extensions: Value;

        match fs::read_to_string(path){
            Err(e) => {
                println!("Error: {} ", e);
                None
            },
            Ok(v) => {
                extensions = serde_json::from_str(&v).unwrap();
                let parsedft: Result<HashMap<String, u64>> = serde_json::from_str(&extensions["ft"].to_string());
                match parsedft {
                    Err(e) => {
                        println!("Error: {} ", e);
                        None
                    },
                    Ok(f) => {
                        let parsedkt: Result<HashMap<String, (Vec<String>, u64)>> = serde_json::from_str(&extensions["known_types"].to_string());
                        match parsedkt {
                            Err(e) => {
                                println!("Error: {} ", e);
                                None
                            },
                            Ok(k) => {
                                let parsedkc: Result<HashMap<String, u64>> = serde_json::from_str(&extensions["known_cat"].to_string());
                                match parsedkc {
                                    Err(e) => {
                                        println!("Error: {} ", e);
                                        None
                                    },
                                    Ok(c) => {
                                        Some((f,k,c))
                                    }
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}