use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use rand_core::OsRng;
use thiserror::Error;

use crate::user::api::create_user_input::CreateUserInput;

use super::user::User;

#[derive(Error, Debug)]
pub enum UserError {
    #[error(transparent)]
    MongoDB(#[from] mongodb::error::Error),

    #[error(transparent)]
    PasswordHashing(#[from] argon2::password_hash::errors::Error),

    #[error("user not found")]
    UserNotFound,
}

#[derive(Clone)]
pub struct UserService {
    pub col: Collection<User>,
}

impl UserService {
    pub async fn create(&self, input: CreateUserInput) -> Result<User, UserError> {
        let password_hash = hash_password(&input)?;
        let result = self
            .col
            .insert_one(
                User {
                    id: None,
                    email: input.email,
                    password: password_hash,
                },
                None,
            )
            .await
            .map_err(UserError::MongoDB)?;

        let result = self
            .col
            .find_one(doc! {"_id": result.inserted_id}, None)
            .await
            .map_err(UserError::MongoDB)?;

        match result {
            Some(user) => Ok(user),
            None => unreachable!("could not find user immediately after creating it"),
        }
    }

    pub async fn find_one_by_id(&self, id: String) -> Result<User, UserError> {
        let result = self
            .col
            .find_one(doc! {"_id": ObjectId::parse_str(id).unwrap()}, None)
            .await
            .map_err(UserError::MongoDB)?;

        match result {
            Some(user) => Ok(user),
            None => Err(UserError::UserNotFound),
        }
    }

    pub async fn find(&self) -> Result<Vec<User>, UserError> {
        let mut cursor = self
            .col
            .find(None, None)
            .await
            .map_err(UserError::MongoDB)?;

        let mut users: Vec<User> = vec![];

        while cursor.advance().await? {
            users.push(cursor.deserialize_current().map_err(UserError::MongoDB)?);
        }
        Ok(users)
    }
}

fn hash_password(input: &CreateUserInput) -> Result<String, UserError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(&input.password.as_bytes(), &salt)
        .map_err(UserError::PasswordHashing)?
        .to_string();
    Ok(password_hash)
}
