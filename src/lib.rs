use csv::{Reader, ReaderBuilder};
use glob::MatchOptions;
use std::ops::Range;
use std::path::PathBuf;
use std::{fs, fs::File};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

const CSV_PATH: &str = "file.csv";
const XML_PATH: &str = "./xml/";

pub fn run() -> MyResult<()> {
    let reader = ReaderBuilder::new().from_path(CSV_PATH)?;
    let mut reader = rule_len_3(reader)?;

    for record in reader.records() {
        let csv_row = record?;

        // should safely unwrap as it was validated above;
        let api_name = csv_row.get(0).expect("api name not readable.");
        let tag = csv_row.get(1).expect("tag name not readable.");
        let data = csv_row.get(2).expect("3rd column not readable.");
        let content_to_replace = replace_special_chars(data);

        let file_path = match_exact_one_file(api_name)?;
        let mut file_content = fs::read_to_string(file_path.into_os_string())?;

        println!("Original: {}", &file_content);

        let range = get_content_range(tag, &file_content)?;
        println!("range: {:?}", range);

        file_content.replace_range(range, content_to_replace.as_str());
        println!("After: {}", &file_content);
    }
    Ok(())
}

fn rule_len_3(mut reader: Reader<File>) -> MyResult<Reader<File>> {
    for record in reader.records() {
        let record = record?;
        // println!("1, {}", record.len());
        if record.len() != 3 {
            return Err(From::from(format!("csv record is not len 3, {:?}", record)));
        }
    }

    Ok(ReaderBuilder::new().from_path(CSV_PATH)?)
}

fn replace_special_chars(content: &str) -> String {
    content.replace("<", "&lt").replace("&", "&amp")
}

fn match_exact_one_file(name: &str) -> MyResult<PathBuf> {
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let glob_value = format!("{}/*{}*.xml", XML_PATH, name);
    let mut file_matches: Vec<_> = glob::glob_with(&glob_value, options)?.collect();

    if file_matches.len() != 1 {
        Err(From::from(format!(
            "keyword '{}' has {} file matched.",
            name,
            file_matches.len()
        )))
    } else {
        Ok(file_matches.remove(0)?)
    }
}

fn get_content_range(tag: &str, content: &str) -> MyResult<Range<usize>> {
    let start = match content.find(format!("<{}>", tag).as_str()) {
        Some(v) => v + tag.len() + 2,
        None => return Err("tag start not found.".into()),
    };

    let end = match content.find(format!("</{}>", tag).as_str()) {
        Some(v) => v,
        None => return Err("tag end not found.".into()),
    };

    let range = start..end;
    Ok(range)
}
