use crate::errors::*;

pub fn loadJson(fpath: &str) -> Result<serde_json::Value> {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    let f = File::open(fpath)?;
    let reader = BufReader::new(f);
    Ok(serde_json::from_reader(reader)?)
}