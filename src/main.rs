use domain::model::roti::Roti;

use crate::domain::port::storage::RotiStorage;
use crate::infra::InMemoryStorageAdapter;

mod domain;
mod infra;

fn main() {
    let roti = Roti::new(None, "test".to_string());

    let mut storage = InMemoryStorageAdapter::new();
    storage.save(roti);
    println!("{:?} ", storage);
}
