
use std::ffi::OsStr;
use std::{fs, collections::HashMap};
use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Jsontypes{
    pub ft: HashMap<String, u64>,
}

impl Jsontypes {
    pub fn new(path: &OsStr) -> Self{
        Jsontypes{
            ft: match Jsontypes::read_exts(path){
                None => HashMap::new(),
                Some(x) => x,
            }
        }
    }

    pub fn read_exts(path: &OsStr) -> Option<HashMap<String, u64>> {
        let extensions: String;

        match fs::read_to_string(path){
            Err(e) => {
                println!("Error: {} ", e);
                None
            },
            Ok(v) => {
                extensions = v;
                let parsed: Result<HashMap<String, u64>> = serde_json::from_str(&extensions);
                match parsed {
                    Err(e) => {
                        println!("Error: {} ", e);
                        None
                    },
                    Ok(v) => {
                        Some(v)
                    }
                }
            }
        }
    }
}