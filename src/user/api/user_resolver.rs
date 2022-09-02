use actix_web::{get, post, web};
use actix_web_validator::{Json, Path};

use crate::{
    common::{json_response::JsonResponse, object_id_input::ObjectIdInput},
    user::service::user_service::UserService,
};

use super::{create_user_input::CreateUserInput, user_model::UserModel};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(find_one_by_id)
            .service(find),
    );
}

#[post("/create")]
async fn create_user(
    user_service: web::Data<UserService>,
    input: Json<CreateUserInput>,
) -> JsonResponse<UserModel> {
    let result = user_service.create(input.into_inner()).await;

    match result {
        Ok(user) => JsonResponse {
            data: Some(UserModel::try_from(user).unwrap()),
            error: None,
        },
        Err(error) => error.into(),
    }
}

#[get("/{id}")]
async fn find_one_by_id(
    user_service: web::Data<UserService>,
    input: Path<ObjectIdInput>,
) -> JsonResponse<UserModel> {
    let result = user_service.find_one_by_id(input.into_inner().id).await;
    match result {
        Ok(user) => JsonResponse {
            data: Some(UserModel::try_from(user).unwrap()),
            error: None,
        },
        Err(error) => error.into(),
    }
}

#[get("")]
async fn find(user_service: web::Data<UserService>) -> JsonResponse<Vec<UserModel>> {
    let result = user_service.find().await;
    match result {
        Ok(users) => JsonResponse {
            data: Some(
                users
                    .into_iter()
                    .map(|e| UserModel::try_from(e).unwrap())
                    .collect(),
            ),
            error: None,
        },
        Err(error) => error.into(),
    }
}
