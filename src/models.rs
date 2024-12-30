use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use utoipa::{Modify, OpenApi, ToSchema};
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

struct SecurityAddon;

#[derive(OpenApi)]
#[openapi(
    tags(
            (name = "todo", description = "Todo management endpoints.")
    ),
    modifiers(&SecurityAddon)
)]
#[derive(Debug, Serialize, ToSchema, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: String,
    pub email: String,
    pub birthdate: String,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(OpenApi)]
#[openapi(
    tags(
            (name = "todo", description = "Todo management endpoints.")
    ),
    modifiers(&SecurityAddon)
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub username_or_email: String,
    pub password: String,
}

#[derive(OpenApi)]
#[openapi(
    tags(
            (name = "todo", description = "Todo management endpoints.")
    ),
    modifiers(&SecurityAddon)
)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserRequest {
    pub email: String,
    pub birthdate: String,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}


impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
        )
    }
}