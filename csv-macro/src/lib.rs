use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use proc_macro::TokenStream;

/// Defines a graph `g` and his associate types from a adjacency matrix represented in a csv.
///
/// Besides the graph `g`, the following items are defined:
///     - `Node` type.
///     - `Weight` type.
///     - `Graph` type.
///     - `NODE_COUNT` constant.
///
/// # Examples
///
/// ```rust,ignore
/// use csv-macro::graph_from_csv;
///
/// graph_from_csv!("problem.csv");
///
/// // Will define
/// const NODE_COUNT: usize = <some_number>;
/// type Node = usize;
/// type Weight = f64;
/// type Graph = [[Weight; NODE_COUNT] NODE_COUNT];
/// const g: Graph = <some_data>;
/// ```
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
                l.push(record.parse().unwrap_or(0.))
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
