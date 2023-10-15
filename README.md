# Transr

A Cli mass-updates a XML tag content from csv input.
I built it for Salesforce translation xml files update.

It expects two optional parameters: 
- a csv-file
- a path to look for xml files

Run `transr -h` to get details.

# Simple logic

Open [sample file](./file.csv),

1. It uses `api_name` column value(e.g.`Customer__c`) to find a matching file (e.g. `Customer__c-en_US.xml`)
2. It uses the `xml_tag` column value(e.g. `help`) to locate a matching tag content(e.g. `<help>foo</help>`) in the xml file
3. It updates the tag content `foo` with the data from the `content` column of the csv

# Preparation

1. prepare `file.csv` with matched columns (api_name,xml_tag,content), check
   [sample file](./file.csv). 
2. prepare translation files in `xml` folder, check [sample files](./xml/)
   
# Use sample data to try out

Download this repo which includes sample data: the `file.csv` and the `xml` folder.
Run `transr` in the root path.
It should update the `help` tag content in the xml file(s) of the xml folder

# How to install

If you have [Cargo](https://www.rust-lang.org/tools/install),
run `cargo install transr` to install it locally.

## To-Do

- Add test scenarios
- Robust error handling rather than stopping
- Generalize the usage?
