use dotent::entry::Entry;

#[test]
fn read_ent_file() {
    let mut entry = Entry::read_file("./tests/data/project.ent").unwrap();
    let project = entry.get_project().unwrap();
    println!("{:#?}", project);
}
