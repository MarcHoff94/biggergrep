use std::fs::File;
use std::io::{Read, Cursor, Error};

pub fn search_xml(file_path: &str, query: &str) -> Result<bool, Error> {

    let mut buf_vec: Vec<u8> = Vec::new();
    let mut file = match File::open(file_path){
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let _ = file.read_to_end(&mut buf_vec);

    let cursor = Cursor::new(buf_vec);
    let mut archive = zip::ZipArchive::new(cursor)?;

    for file_index in 0..archive.len() { 
        let mut xmlfile = archive.by_index(file_index)?;
        let mut contents = String::new();
        if  xmlfile.name().contains(".xml") {
            xmlfile.read_to_string(&mut contents)?;
            if contents.contains(query) {
                return Ok(true)
            }
        }
    }
    Ok(false)
}