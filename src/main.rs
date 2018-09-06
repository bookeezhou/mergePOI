#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

mod can;


use std::collections::HashMap;

fn main() {
    let mut can_convert = can::CanConvert {nation_coding_dic : HashMap::with_capacity(752233)};

    can_convert.merge_address_full_name();
}