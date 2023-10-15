# Transr

A Cli mass-updates a XML tag content from csv input.

Run `transr -h` to get details.

# How it works

Open [sample file](./file.csv), it does simple logic as below.

```
Loop all rows in csv {
   use column1(`Target_Customer_Type__c`) to locate a partial matching xml file (e.g.
   `ABCTarget_Customer_Type__cDEF.xml`)

   use column2(`help`) to find the tag content in the xml (e.g. `<help>foo</help>`)

   update tag content(`foo`) using column3 value(`info_to_update!`), and save
   the xml file
}
```

# Preparation

1. prepare `file.csv` with matched columns (api_name,xml_tag,content), check
   [sample file](./file.csv). 
2. prepare translation files in `xml` folder, check [sample folder](./xml/)
   
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
