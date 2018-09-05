
// 加载行政区划表和 POI 数据

use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::collections::HashMap;


pub struct CanConvert {
    pub nation_coding_dic: HashMap<i32, Vec<String>>,
}

impl CanConvert {

    fn loadFile(& mut self) {
        let f = File::open("data/nationAddressCode.txt").unwrap();
        let f = BufReader::new(f);

        for line in f.lines() {
            let v: Vec<String> = line.unwrap().split('/').map(|m| m.to_string() ).collect();
            if v.len() == 12 {
                self.nation_coding_dic.insert(v[0].parse().unwrap(), v);
            }
        }

    }


    pub fn mergeAddressFullName(&mut self) {

        self.loadFile();

        let mut fullMergeName: Vec<String> = Vec::with_capacity(10);
        let mut level = 0;
        let mut pid = 0;

        for val in self.nation_coding_dic.values_mut() {
            level = val[2].parse().unwrap();
            fullMergeName.push(val[6].to_string());
            pid = val[1].parse().unwrap();
            level -= 1;

            
            while level >= 0 {
                let item = self.nation_coding_dic.get(&pid).unwrap();
                fullMergeName.push(item[6].to_string()); // get name not shor_name
                pid = item[1].parse().unwrap();
                level -= 1;
            }
            

            // 省市区县合并
            {
            let fullMergeNameSlice = fullMergeName.as_mut_slice();
            fullMergeNameSlice.reverse();
            val[8] = fullMergeNameSlice.join("-");
            }

            fullMergeName.clear();
            
        }

        self.saveFile();
    }

    fn saveFile(&self) {
        let f = File::create("data/nac.csv").unwrap();
        let f = BufWriter::new(f);
        
        for val in self.nation_coding_dic.values() {
            let val_to_slice = val.as_slice();
            //writeln!(f, val_to_slice.join(","));
        }
        
    }
}





