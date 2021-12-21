/*
 *                  Cadence Data Soft Pvt. Ltd.
 *
*/

use std::{collections::HashMap, path::Path};

#[path = "./jsontypes.rs"]
mod jsontypes;

pub struct FileTypes {
    pub ft: HashMap<String, u64>,
    pub uft: HashMap<String, u64>,
    pub known_types: HashMap<String, (Vec<String>, u64)>,
    pub known_cat: HashMap<String, u64>,
    pub others: u64,
}

impl FileTypes {
    pub fn new() -> Self {
        let jt = jsontypes::Jsontypes::new(Path::new("D:\\Projects\\RustProgramming\\disk-space\\find-empty-folders\\exte.json").as_os_str());
        FileTypes{
            ft: jt.ft,
            uft: HashMap::new(),
            known_types: jt.known_types,
            known_cat: jt.known_cat,
            others: 0,
        }   
    }

    pub fn inc(& mut self, ext: String) -> () {
        let res = self.ft.get_mut(&ext);
        match res {
            Some(val) => {
                *val = *val + 1;
                self.inc_category(ext);
            },
            None => {
                self.uft.insert(ext, 1);
                self.others += 1;
            }
        }
    }

    fn inc_category(&mut self, ext: String ){
        for (k, (x, y)) in &mut self.known_types{
            if x.contains(&ext) {
                *y = 1 + *y;
                *self.known_cat.get_mut(k).unwrap() += 1;
                break;
            }
        }
    }
}
