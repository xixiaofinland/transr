use clap::{Arg, ArgAction, Command};
use csv::{Reader, ReaderBuilder, StringRecord};
use glob::MatchOptions;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::{fs, fs::File};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

const VERSION: &str = "0.1.1";
const CSV_DEFAULT: &str = "file.csv";
const XML_DEFAULT_PATH: &str = "./xml";

#[derive(Debug)]
pub struct Config {
    in_file: String,
    xml_path: String,
    dry_run: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("transr")
        .version(VERSION)
        .author("Xi Xiao <tdxiaoxi2@gmail.com>")
        .about("Update xml content from csv")
        .arg(
            Arg::new("csv")
                .value_name("CSV")
                .short('c')
                .long("csv-input")
                .help("CSV input file")
                .default_value(CSV_DEFAULT),
        )
        .arg(
            Arg::new("xml_path")
                .value_name("XML_PATH")
                .short('x')
                .long("xml-path")
                .help("The path including xml files to be updated")
                .default_value(XML_DEFAULT_PATH),
        )
        .arg(
            Arg::new("dry_run")
                .value_name("DRY_RUN")
                .short('d')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Print result to Stdout only. Great for trying out without touching xml content"),
        )
        .get_matches();

    let config = Config {
        in_file: matches.get_one("csv").cloned().unwrap(),
        xml_path: matches.get_one("xml_path").cloned().unwrap(),
        dry_run: matches.get_flag("dry_run"),
    };
    // println!("{:?}", config);

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    validate_xml_path(&config.xml_path)?;
    let mut reader = validate_and_get_input(&config.in_file)?;

    for record in reader.records() {
        let (api_name, tag, to_replace) = parse(record?);

        let file_path = match_exact_one_file(&api_name, &config.xml_path)?.into_os_string();
        let mut file_content = fs::read_to_string(&file_path)?;
        // println!("Original: {}", &file_content);

        let range = get_content_range(&tag, &file_content)?;
        // println!("range: {:?}", range);

        file_content.replace_range(range, to_replace.as_str());

        if config.dry_run {
            println!("{}", &file_content);
        } else {
            fs::write(file_path, &file_content)?;
        }
    }
    Ok(())
}

fn validate_and_get_input(in_file: &str) -> MyResult<Reader<File>> {
    let mut reader = match ReaderBuilder::new().from_path(in_file) {
        Err(_) => return Err(format!("Read file failed: {}", in_file).into()),
        Ok(v) => v,
    };

    for record in reader.records() {
        let record = record?;
        if record.len() != 3 {
            return Err(From::from(format!("csv record is not len 3, {:?}", record)));
        }
    }

    Ok(ReaderBuilder::new().from_path(in_file)?)
}

fn validate_xml_path(path: &str) -> MyResult<()> {
    if let false = Path::new(path).exists() {
        return Err(format!("xml folder doesn't exist: '{}'", path).into());
    }
    Ok(())
}

fn parse(s: StringRecord) -> (String, String, String) {
    (
        s.get(0).expect("1st column not readable.").to_string(),
        s.get(1).expect("2nd column not readable.").to_string(),
        replace_special_chars(s.get(2).expect("3rd column not readable.")),
    )
}

fn replace_special_chars(content: &str) -> String {
    content.replace("<", "&lt").replace("&", "&amp")
}

fn match_exact_one_file(name: &str, xml_path: &str) -> MyResult<PathBuf> {
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let glob_value = format!("{}/*{}*.xml", xml_path, name);
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
        None => return Err(format!("<{}> tag not found.\nFile:\n{}", tag, content).into()),
    };

    let end = match content.find(format!("</{}>", tag).as_str()) {
        Some(v) => v,
        None => return Err(format!("</{}> tag not found.\nFile:\n{}", tag, content).into()),
    };

    let range = start..end;
    Ok(range)
}
