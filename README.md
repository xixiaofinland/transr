# Transr

A CLI command that I use to mass update Salesforce translations in metadata
files.

# Simple logic

- It locates xml file according to predefined name in the csv (e.g.`Customer__c` keyword to
  locate `Customer__c-en_US.xml`)
- It locates the tag content (e.g. `<help>...</help>`) in the xml file
- It updates the tag content with predefined data in the csv row

Note. `trans` reads the XML content into a String, so don't use it for a very large XML
content.

# Preparation

1. prepare `file.csv` with matched columns (api_name,xml_tag,content), check
   [sample file](./file.csv). 
2. prepare translation files in `xml` folder, check [sample files](./xml/)
   
# Use sample data to demo

Download this repo which includes sample data: the `file.csv` and the `xml` folder.
Run `transr` in the root path.
It should update the `help` tag content in the xml file(s) of the xml folder

# Execution logic

1. Run `transr` in the current path. It expects both the `file.csv` and the
   `./xml` folder in place, otherwise throws error.
2. It exepects all rows in `file.csv` have exactly 3 columns (api_name,xml_tag,content)
3. It uses `api_name` to match files in xml folder, and expect exact one file
   match
4. It locates the `xml_tag` content, and uses `content` to replace existing
   content

# How to install

You need to have [Cargo command](https://www.rust-lang.org/tools/install) to install the tool
Run `cargo install transr` to install it locally.

## To-Do?

- Add test scenarios
- Clap support to allow pass parameters
- Better error scenario handling
- Generalize the usage
