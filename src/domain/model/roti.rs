use uuid::Uuid;

#[derive(Debug)]
pub struct Roti {
    pub id: RotiId,
    pub name: String,
    pub status: RotiStatus,
}

pub type RotiId = String;

#[derive(Debug, PartialEq)]
pub enum RotiStatus {
    Draft,
    DeletedAt(String),
    PublishedAt(String),
}

impl Roti {
    pub fn new(id: Option<RotiId>, name: String) -> Self {
        Roti {
            id: id.unwrap_or(Uuid::new_v4().to_string()),
            name,
            status: RotiStatus::Draft,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_roti_is_draft() {
        let roti = Roti::new(Some("123".to_string()), "test".to_string());

        assert_eq!(RotiStatus::Draft, roti.status);
    }
}
