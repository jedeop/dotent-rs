use dotent::{entry::Entry, error::Result};

#[test]
fn read_basic_ent_file() {
    let entry = Entry::read_file("./tests/data/project.ent").unwrap();
    let project = entry.project();
    println!("{:#?}", project);
    assert_eq!("220705_작품", project.name);
}

#[test]
fn read_ent_file_with_asset() {
    let entry = Entry::read_file("./tests/data/project_asset.ent").unwrap();
    let assets = entry.assets();

    let keys = assets.keys();

    let assets = [
        "d727iwfklcd9f1nj29m7a2c94335v99z",
        "v28qd1qalcd9f1nj29m7a2c94335wmnx",
        "h50rd64tlcd9f1nj29m7a2c94335x2jz",
        "p5th6yuulcd9f1nj29m7a2c94335xotg",
    ];

    for key in keys {
        assert!(assets.contains(&&key[..]));
    }
}

#[test]
fn unpack_ent_file() -> Result<()> {
    Entry::unpack("./tests/data/project.ent", "./temp")
}
