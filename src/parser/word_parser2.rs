use std::fs::File;
use std::io::Read;

use docx_rs::read_docx;
use serde_json::Value;


fn read_bytes_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    File::open(file_name)?.read_to_end(&mut buf)?;
    Ok(buf) 
}

fn parse_docx(file_name: &str) -> anyhow::Result<Vec<String>> {
    let data: Value = serde_json::from_str(&read_docx(&read_bytes_to_vec(file_name)?)?.json())?;
    let mut text_data: Vec<String> = Vec::new();

    if let Some(children) = data["document"]["children"].as_array() {
        children.iter().for_each(|child| read_children(child, &mut text_data));
    }
    Ok(text_data)
}

fn read_children(node: &Value, text_vec: & mut Vec<String>) {
    if let Some(children) = node["data"]["children"].as_array() {
        children.iter().for_each(|child| {
            if child["type"] != "text" {
                read_children(child, text_vec);
            } else {
                text_vec.push(child["data"]["text"].to_string());
            }
        })
    }
}

pub fn search_wordfile(file_path: &str, query: &str) -> anyhow::Result<bool> {
    let text_data = parse_docx(file_path)?;

    let matched_text: Vec<_> = text_data.iter().filter(|text| text.contains(query)).collect();
    if matched_text.len() > 0 {
        return Ok(true);
    }
    Ok(false)

}