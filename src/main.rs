mod can;


use std::collections::HashMap;

fn main() {
    let mut can_convert = can::CanConvert {nation_coding_dic : HashMap::with_capacity(752233)};

    can_convert.mergeAddressFullName();
}