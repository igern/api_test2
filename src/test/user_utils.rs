use crate::common::json_response::JsonResponse;
use crate::user::api::{create_user_input::CreateUserInput, user_model::UserModel};
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, Error};

use super::helpers::is_successful;

pub async fn create_user<S>(app: &S, input: &CreateUserInput) -> UserModel
where
    S: Service<Request, Response = ServiceResponse, Error = Error>,
{
    let req = test::TestRequest::post()
        .uri("/users/create")
        .set_json(&input)
        .to_request();
    let json: JsonResponse<UserModel> = test::call_and_read_body_json(&app, req).await;
    is_successful(json)
}

pub async fn find_one<S>(app: &S, id: String) -> UserModel
where
    S: Service<Request, Response = ServiceResponse, Error = Error>,
{
    let req = test::TestRequest::get()
        .uri(&format!("/users/{}", id))
        .to_request();
    let json: JsonResponse<UserModel> = test::call_and_read_body_json(&app, req).await;
    is_successful(json)
}
