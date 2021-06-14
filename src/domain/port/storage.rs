use crate::domain::model::roti::Roti;

pub trait RotiStorage {
    fn save(&mut self, roti: Roti);
}
