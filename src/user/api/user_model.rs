use serde::{Deserialize, Serialize};

use crate::user::service::user::User;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserModel {
    pub id: String,
    pub email: String,
}

impl TryFrom<User> for UserModel {
    type Error = &'static str;

    fn try_from(value: User) -> Result<Self, Self::Error> {
        match value.id {
            Some(id) => Ok(UserModel {
                id: id.to_string(),
                email: value.email,
            }),
            None => Err("UserDocument is missing id"),
        }
    }
}
