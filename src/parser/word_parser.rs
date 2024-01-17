use std::fs::File;
use std::io::Read;

use docx_rs::read_docx;
use serde_json::Value;


fn read_bytes_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    File::open(file_name)?.read_to_end(&mut buf)?;
    Ok(buf) 
}

pub fn search_wordfile(file_name: &str, query: &str) -> anyhow::Result<bool> {
    let data: Value = serde_json::from_str(&read_docx(&read_bytes_to_vec(file_name)?)?.json())?;
    if read_children(&data, query) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn read_children(data: &Value, query: &str) -> bool {
    if let Some(children) = data["data"]["children"].as_array() {
        for child in children.iter() {
            if child["type"] != "text" {
                if read_children(child, query) {
                    return true;
                }
            } else if child["data"]["text"].to_string().contains(query) {
                return true;
            }
        }
    }
    false
}

