#[cfg(test)]
mod tests {
    use actix_web::test;

    use crate::{
        common::json_response::JsonResponse,
        test::{
            helpers::{is_error, is_successful, setup, MOCK_MONGO_ID},
            user_utils::find_one,
        },
        user::{
            api::{create_user_input::CreateUserInput, user_model::UserModel},
            service::user_service::UserError,
        },
    };

    #[actix_web::test]
    async fn create_user() {
        let app = setup().await;

        let input = CreateUserInput {
            email: String::from("test@test.com"),
            password: String::from("secret1234"),
        };

        let req = test::TestRequest::post()
            .uri("/users/create")
            .set_json(&input)
            .to_request();

        let json: JsonResponse<UserModel> = test::call_and_read_body_json(&app, req).await;
        let data = is_successful(json);
        assert_eq!(data.email, input.email)
    }

    #[actix_web::test]
    async fn create_user_invalid_email() {
        let app = setup().await;

        let input = CreateUserInput {
            email: String::from("testtest.com"),
            password: String::from("secret1234"),
        };

        let req = test::TestRequest::post()
            .uri("/users/create")
            .set_json(&input)
            .to_request();

        let json: JsonResponse<UserModel> = test::call_and_read_body_json(&app, req).await;
        let error = is_error(json);
        assert_eq!(error.code, "json error");
    }

    #[actix_web::test]
    async fn create_user_invalid_password() {
        let app = setup().await;

        let input = CreateUserInput {
            email: String::from("test@test.com"),
            password: String::from("1234"),
        };

        let req = test::TestRequest::post()
            .uri("/users/create")
            .set_json(&input)
            .to_request();

        let json: JsonResponse<UserModel> = test::call_and_read_body_json(&app, req).await;
        let error = is_error(json);
        assert_eq!(error.code, "json error");
    }

    #[actix_web::test]
    async fn find_one_by_id_user_not_found() {
        let app = setup().await;

        let req = test::TestRequest::get()
            .uri(&format!("/users/{}", MOCK_MONGO_ID))
            .to_request();

        let json: JsonResponse<UserModel> = test::call_and_read_body_json(&app, req).await;
        let error = is_error(json);
        assert_eq!(error.code, UserError::UserNotFound.to_string())
    }

    #[actix_web::test]
    async fn find_one_by_id() {
        let app = setup().await;

        let user = crate::test::user_utils::create_user(
            &app,
            &CreateUserInput {
                email: String::from("test@test.com"),
                password: String::from("secret1234"),
            },
        )
        .await;

        let req = test::TestRequest::get()
            .uri(&format!("/users/{}", user.id))
            .to_request();

        let json: JsonResponse<UserModel> = test::call_and_read_body_json(&app, req).await;
        let data = is_successful(json);
        assert_eq!(data, user)
    }

    #[actix_web::test]
    async fn find() {
        let app = setup().await;

        let req = test::TestRequest::get()
            .uri(&format!("/users"))
            .to_request();

        let json: JsonResponse<Vec<UserModel>> = test::call_and_read_body_json(&app, req).await;
        let data = is_successful(json);
        if let Some(first) = data.first() {
            let user = find_one(&app, first.id.to_string()).await;
            assert_eq!(user.email, "jonas@littlegiants.dk");
        }
    }
}
