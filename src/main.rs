mod auth_service;
pub mod models;
pub mod schema;
mod auth_middleware;
mod auth;

use utoipa::IntoParams;
use actix_web::{middleware, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use utoipa::{Modify, OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use crate::auth::{create_user, health, login};
use crate::auth_middleware::JwtMiddleware;

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
        App::new()
            .service(health)
            .service(login)
            .service(create_user)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .wrap(middleware::Logger::default())
            .wrap(JwtMiddleware { secret: secret.clone() })
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
