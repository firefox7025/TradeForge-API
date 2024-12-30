use actix_web::{post, get, web, HttpResponse, Responder};
use crate::auth_service::{create_jwt, establish_connection, verify_login, insert_new_user};
use bcrypt::{hash, DEFAULT_COST};
use crate::models::{Login, NewUserRequest};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[post("/users/login")]
pub async fn login(request: web::Json<Login>) -> impl Responder {
    let login = Login {
        username_or_email: request.username_or_email.clone(),
        password: request.password.clone(),
    };
    let secret = "my-secret".to_string();
    let connection = &mut establish_connection();
    let login_result = verify_login(connection, login).await;
    match login_result {
        Ok(user) => {
            HttpResponse::Ok().json(create_jwt(user, &*secret))
        },
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[post("/users/create")]
pub async fn create_user(creation_request: web::Json<NewUserRequest>) -> impl Responder {
    let new_user = NewUserRequest {
        email: creation_request.email.clone(),
        birthdate: creation_request.birthdate.clone(),
        firstname: creation_request.firstname.clone(),
        lastname: creation_request.lastname.clone(),
        username: creation_request.username.clone(),
        password: hash(creation_request.password.clone(), DEFAULT_COST).unwrap(),
    };
    let connection = &mut establish_connection();
    let user = insert_new_user(connection, new_user);
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}