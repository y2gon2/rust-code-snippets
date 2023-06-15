// p137 HashMap

use std::io::Result;
use std::collections::HashMap;

const V_DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C";

fn main() -> Result<()> {
    let mut map = HashMap::<char, usize>::new();

    for c in V_DATA.chars() {
        if c != ',' {
            map.entry(c).and_modify(|v| {*v += 1}).or_insert(1);
        }
    }

    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }

    Ok(())
}