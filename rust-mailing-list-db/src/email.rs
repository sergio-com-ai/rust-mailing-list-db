use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
    pub id: Uuid,
    pub email: String,
}

impl Email {
    pub fn new(email: String) -> Self {
        Email {
            id: Uuid::new_v4(),
            email,
        }
    }
}

