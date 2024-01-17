use std::fs;
use std::error::Error;
use std::env;
use walkdir::{WalkDir, DirEntry};
use std::os::windows::fs::MetadataExt;
use parser::pdf_parser::search_pdf;
use parser::xml_parser::search_xml;

pub mod parser;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let dirwalker = WalkDir::new(config.directory_path).into_iter();
    let mut result_vec: Vec<String> = Vec::new();
    let mut failed_files_vec: Vec<String> = Vec::new();
    for entry_result in dirwalker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry_result.unwrap();
        let file_name = match entry.file_name().to_str() {
            Some(file) => file,
            None => continue,
        };
        let filetype = file_name.split(".");
        //println!("{}", file_name); 
        match filetype.last().unwrap() {
            "pdf" => match search_pdf(file_name, &config.query) {
                Ok(true) => result_vec.push(
                    String::from(entry.path()
                        .to_str()
                        .unwrap()
                    )),
                Err(_) => failed_files_vec.push(String::from(file_name)),
                _ => continue,
            },
            "docx" | "xlsx" | "pptx" => match search_xml(file_name, &config.query) {
                Ok(true) => result_vec.push(
                    String::from(entry.path()
                        .to_str()
                        .unwrap()
                    )),
                Err(_) => failed_files_vec.push(String::from(file_name)),
                _ => continue,
            },
            _ => continue,
        }
    }
    println!("{:?}", result_vec);
    println!("{:?}", failed_files_vec);
    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    println!("{}", query);
    contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}
pub struct Config {
    pub query: String,
    pub directory_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string"),
        };
        let directory_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a filepath"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query, 
            directory_path,
            ignore_case,
        })
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    if cfg!(target_os = "windows") {
        // Check if the FILE_ATTRIBUTE_HIDDEN flag is set
        if let Ok(metadata) = fs::metadata(entry.path()) {
            metadata.file_attributes() & 2 != 0 // Bit 1 represents FILE_ATTRIBUTE_HIDDEN
        } else {
            false
        }
    } else {
        // For non-Windows systems, check if the file name starts with a dot
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}