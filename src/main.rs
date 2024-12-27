mod auth_service;
pub mod models;
pub mod schema;
mod auth_middleware;

use crate::auth_service::establish_connection;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use crate::auth_middleware::JwtMiddleware;

#[derive(Serialize, Deserialize, Debug)]
struct Login {
    username_or_email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewUserRequest {
    email: String,
    birthdate: String,
    firstname: String,
    lastname: String,
    username: String,
    password: String,
}




#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[post("/users/verify")]
async fn login(request: web::Json<Login>) -> impl Responder {
    let login = Login {
        username_or_email: request.username_or_email.clone(),
        password: request.password.clone(),
    };
    let connection = &mut establish_connection();
    let login = auth_service::verify_login(connection, login).await;
    HttpResponse::Ok().body(login.to_string())
}

#[post("/users/create")]
async fn create_user(creation_request: web::Json<NewUserRequest>) -> impl Responder {
    let new_user = NewUserRequest {
        email: creation_request.email.clone(),
        birthdate: creation_request.birthdate.clone(),
        firstname: creation_request.firstname.clone(),
        lastname: creation_request.lastname.clone(),
        username: creation_request.username.clone(),
        password: hash(creation_request.password.clone(), DEFAULT_COST).unwrap(),
    };
    let connection = &mut establish_connection();
    auth_service::insert_new_user(connection, new_user);
    HttpResponse::Ok().body("User created!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let secret = "my-secret".to_string();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }


    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .wrap(middleware::Logger::default())
            .wrap(JwtMiddleware { secret: secret.clone() })
            .service(health)
            .service(login)
            .service(create_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
