use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use proc_macro::TokenStream;

#[proc_macro]
pub fn graph_from_csv(item: TokenStream) -> TokenStream {
    let item_str = item.to_string();
    let path = item_str.trim_matches('"');
    let file = BufReader::new(File::open(path).unwrap());
    let mut data = Vec::new();

    for line in file.lines().skip(1) {
        data.push(Vec::new());
        for record in line.unwrap().split(',').skip(1) {
            if let Some(l) = data.last_mut() {
                l.push(record.trim_matches('"').parse().unwrap_or(0.))
            }
        }
    }

    let mut ret = String::new();
    ret.push_str(format!("const NODE_COUNT: usize = {};", data.len()).as_str());
    ret.push_str("type Node = usize;");
    ret.push_str("type Weight = f64;");
    ret.push_str("type Graph = [[Weight; NODE_COUNT]; NODE_COUNT];");
    ret.push_str(format!("const g: Graph = {:?};", data).as_str());
    ret.parse().unwrap()
}
