

// 加载行政区划表和 POI 数据

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::collections::HashMap;
use std::cell::RefCell;


pub struct CanConvert {
    pub nation_coding_dic: HashMap<i32, RefCell<Vec<String>>>,
}

impl CanConvert {

    fn load_file(& mut self) {
        let f = File::open("data/nationAddressCode.txt").unwrap();
        let f = BufReader::new(f);

        for line in f.lines() {
            let v: Vec<String> = line.unwrap().split('/').map(|m| m.to_string() ).collect();
            if v.len() == 12 {
                self.nation_coding_dic.insert(v[0].parse().unwrap(), RefCell::new(v));
            }
        }

    }


    pub fn merge_address_full_name(&mut self) {

        self.load_file();

        let mut fullMergeName: Vec<String> = Vec::with_capacity(10);
        let mut level = 0;
        let mut pid = 0;

        for val in self.nation_coding_dic.values() {
            level = val.borrow_mut()[2].parse().unwrap();
            fullMergeName.push(val.borrow_mut()[6].to_string());
            pid = val.borrow_mut()[1].parse().unwrap();
            level -= 1;

            
            while level >= 0 {
                let item = self.nation_coding_dic.get(&pid).unwrap();
                fullMergeName.push(item.borrow_mut()[6].to_string()); // get name not shor_name
                pid = item.borrow_mut()[1].parse().unwrap();
                level -= 1;
            }
            

            // 省市区县合并
            {
            let fullMergeNameSlice = fullMergeName.as_mut_slice();
            fullMergeNameSlice.reverse();
            val.borrow_mut()[8] = fullMergeNameSlice.join("-");
            }

            fullMergeName.clear();
            
        }

        self.save_file();
    }

    fn save_file(&self) {
        let f = File::create("data/nac.csv").unwrap();
        let mut f = BufWriter::new(f);
        
        for val in self.nation_coding_dic.values() {
            let refVal = val.borrow();
            let val_to_slice = refVal.as_slice();
            writeln!(f, "{}", val_to_slice.join("/"));
        }
        
    }
}





