# dotent-rs

A crate for handling [Entry](https://playentry.org/) File(`.ent`).

## Usage

```rust
use dotent::entry::Entry;

fn main() {
    // Read Entry file.
    let mut entry = Entry::read_file("./path/to/file.ent").unwrap();
    
    // Get the project data of Entry file.
    let project = entry.project();
    println!("{}", project.name);

    // Get assets in Entry file.
    let assets = entry.assets();
    let keys = assets.keys();
    println!("{:#?}", keys);
}
```
