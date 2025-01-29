mod auth;
mod auth_middleware;
mod auth_service;
pub mod models;
pub mod schema;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::auth::{create_user, health, login};
use crate::auth_middleware::JwtMiddleware;
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret = "my-secret".to_string();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .expose_any_header()
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(health)
            .service(login)
            .service(create_user)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .wrap(middleware::Logger::default())
            .wrap(JwtMiddleware {
                secret: secret.clone(),
            })
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
