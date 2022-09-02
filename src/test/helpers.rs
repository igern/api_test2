use std::{env, fmt::Debug};

use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    error, test, web, App, Error, HttpResponse,
};
use dotenv::dotenv;
use mongodb::Client;

use crate::{
    common::{json_error_response::JsonErrorResponse, json_response::JsonResponse},
    user::{api::user_resolver::user_routes, service::user_service::UserService},
};

pub const MOCK_MONGO_ID: &'static str = "507f1f77bcf86cd799439011";

pub async fn setup() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    dotenv().ok();

    let mongodb_uri = env::var("MONGODB_URI").expect("Please set MONGODB_URI in .env");
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("could not connect to database");
    let db = client.database("Rust");
    let user_service = UserService {
        col: db.collection("user"),
    };

    let json_validator_cfg =
        actix_web_validator::JsonConfig::default().error_handler(|err, _req| {
            let error_response: JsonResponse<()> = JsonResponse {
                data: None,
                error: Some(JsonErrorResponse {
                    code: String::from("json error"),
                    message: Some(err.to_string()),
                }),
            };

            error::InternalError::from_response("", HttpResponse::Ok().json(error_response).into())
                .into()
        });

    let path_validator_cfg =
        actix_web_validator::PathConfig::default().error_handler(|err, _req| {
            let error_response: JsonResponse<()> = JsonResponse {
                data: None,
                error: Some(JsonErrorResponse {
                    code: String::from("json error"),
                    message: Some(err.to_string()),
                }),
            };

            error::InternalError::from_response("", HttpResponse::Ok().json(error_response).into())
                .into()
        });

    test::init_service(
        App::new()
            .app_data(path_validator_cfg.clone())
            .app_data(json_validator_cfg.clone())
            
            .app_data(web::Data::new(user_service.clone()))
            .configure(user_routes),
    )
    .await
}

pub fn is_successful<T>(json_response: JsonResponse<T>) -> T
where
    T: Debug,
{
    match json_response.data {
        Some(data) => data,
        None => panic!("Data not found. {:?}", json_response),
    }
}

pub fn is_error<T>(json_response: JsonResponse<T>) -> JsonErrorResponse
where
    T: Debug,
{
    match json_response.error {
        Some(error) => error,
        None => panic!("Error not found. {:?}", json_response),
    }
}
