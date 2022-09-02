use std::env;

use actix_web::{error, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use mongodb::Client;

use crate::{
    common::{json_error_response::JsonErrorResponse, json_response::JsonResponse},
    user::{api::user_resolver::user_routes, service::user_service::UserService},
};

pub async fn server() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("Please set HOST in .env");
    let port = env::var("PORT").expect("Please set PORT in .env");

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
                    code: String::from("path error"),
                    message: Some(err.to_string()),
                }),
            };

            error::InternalError::from_response("", HttpResponse::Ok().json(error_response).into())
                .into()
        });

    HttpServer::new(move || {
        App::new()
            .app_data(path_validator_cfg.clone())
            .app_data(json_validator_cfg.clone())
            .app_data(web::Data::new(user_service.clone()))
            .configure(user_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
