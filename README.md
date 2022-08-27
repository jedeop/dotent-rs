# dotent-rs

A crate for handling [Entry](https://playentry.org/) File(`.ent`).

## Usage

```rs
use dotent::entry::Entry;

fn main() {
    // Read Entry file and unpack it at temp directory.
    // If you want to specify unpack path, use `Entry::read_file_with_unpack_path` instead.
    let mut entry = Entry::read_file("./path/to/file.ent").unwrap();
    
    // Get the project data of Entry file.
    let project = entry.get_project().unwrap();
    
    println!("{}", project.name);
}
```
