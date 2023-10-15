# Transr

A CLI command that I use to mass update Salesforce translations in metadata
files.

Note. it reads XML content into a String, so don't use it for large size XML
file.

# Preparation

1. prepare `file.csv` with matched columns (api_name,xml_tag,content), check
   [sample file](./file.csv). 
2. prepare translation files in `xml` folder, check [sample files](./xml/)
   
# Execution logic

1. Run `trasnr` in the current path. It expects both the `file.csv` and the
   `./xml` folder in place, otherwise throws error.
2. It exepects all rows in `file.csv` have 3 columns (api_name,xml_tag,content)
3. It uses `api_name` to match files in xml folder, and expect exact one file
   match
4. It locates the `xml_tag` content, and uses `content` to replace existing
   content

# Use sample data to demo

Run `transr` in the same path of the `file.csv` and `xml` folder included in
this repo.

It will update the `help` tag content in the xml file.

# How to install

You need to have [Cargo command](https://www.rust-lang.org/tools/install) to install the tool
Run `cargo install transr` to install it locally.

## To-Do?

- Add test scenarios
- Clap support to allow pass parameters
- Better error scenario handling
- Generalize the usage
