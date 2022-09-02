use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct ObjectIdInput {
    #[validate(custom = "validate_object_id")]
    pub id: String,
}

fn validate_object_id(id: &str) -> Result<(), ValidationError> {
    match ObjectId::parse_str(id) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("object_id_error")),
    }
}
