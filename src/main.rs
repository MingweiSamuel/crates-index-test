use crates_index::GitIndex;

fn main() {
    let index = GitIndex::new_cargo_default().unwrap();
    let c = index.crate_("hydroflow").unwrap();
    println!("{:#?}", c.versions().iter().map(|v| v.version()).collect::<Vec<_>>());
}
