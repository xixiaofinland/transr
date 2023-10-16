# Transr

A Cli mass-updates a XML tag content from csv input.

Run `transr -h` to get details.

# How it works

Open [sample file](./file.csv), it does simple logic as below.

```
Loop all rows in csv {
   column1(`Target_Customer_Type__c`) -- match -> xml file (`ABCTarget_Customer_Type__cDEF.xml`)

   column2(`help`) -- find --> (`<help>foo</help>`) in the matched xml file

   column3(`info_to_update!`) -- update --> (`<help>info_to_update!</help>`) and save
   the xml file
}
```

# How to use it

1. prepare `file.csv` with matched columns (api_name,xml_tag,content), check
   [sample file](./file.csv)
2. prepare translation files in `xml` folder, check [sample folder](./xml/)
3. use `-d` to dry run until all file updates are as expected
4. run it without `-d` to write into xml files
   
# How to install

If you have [Cargo](https://www.rust-lang.org/tools/install),
run `cargo install transr` to install it locally.

## To-Do

- Add test scenarios
- Generalize the usage?
