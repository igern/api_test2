use std::fmt::Debug;

use actix_http::body::BoxBody;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::user::service::user_service::UserError;

use super::json_error_response::JsonErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse<T> {
    pub data: Option<T>,
    pub error: Option<JsonErrorResponse>,
}

impl<T> Responder for JsonResponse<T>
where
    T: Serialize,
{
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl<T> From<UserError> for JsonResponse<T> {
    fn from(error: UserError) -> Self {
        JsonResponse {
            data: None,
            error: Some(JsonErrorResponse {
                code: error.to_string(),
                message: None,
            }),
        }
    }
}
