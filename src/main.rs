use crates_index::{GitIndex, SparseIndex};

fn main() {
    {
        let mut index = GitIndex::new_cargo_default().unwrap();
        println!("URL {}", index.url());
        index.update().unwrap();
        let c = index.crate_("gitoxide").unwrap();
        println!(
            "{:#?}",
            c.versions().iter().map(|v| v.version()).collect::<Vec<_>>()
        );
    }
    {
        let index = SparseIndex::new_cargo_default().unwrap();
        let c = index.crate_from_cache("gitoxide").unwrap();
        println!(
            "{:#?}",
            c.versions().iter().map(|v| v.version()).collect::<Vec<_>>()
        );
    }
}
