use crate::domain::model::roti::Roti;
use crate::domain::port::storage::RotiStorage;

#[derive(Debug)]
pub struct RotiStored {
    id: String,
    name: String,
}

#[derive(Debug)]
pub struct InMemoryStorageAdapter {
    rotis: Vec<RotiStored>,
}

impl InMemoryStorageAdapter {
    pub fn new() -> InMemoryStorageAdapter {
        InMemoryStorageAdapter { rotis: Vec::new() }
    }
}

impl RotiStorage for InMemoryStorageAdapter {
    fn save(&mut self, roti: Roti) {
        self.rotis.push(RotiStored::from(roti));
    }
}

impl From<Roti> for RotiStored {
    fn from(r: Roti) -> Self {
        RotiStored {
            id: r.id,
            name: r.name,
        }
    }
}
