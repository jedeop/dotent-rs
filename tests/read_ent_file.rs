use dotent::entry::Entry;

#[test]
fn read_ent_file() {
    let mut entry = Entry::read_file("./tests/data/project.ent").unwrap();
    let project = entry.get_project().unwrap();
    println!("{}", project.name);
}

#[test]
fn read_ent_file_with_unpack_path() {
    let mut entry =
        Entry::read_file_with_unpack_path("./tests/data/project.ent", "./temp/").unwrap();
    let project = entry.get_project().unwrap();
    println!("{:#?}", project);
}
