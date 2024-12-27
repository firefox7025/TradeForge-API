use std::env;
use diesel::{Connection, Insertable, PgConnection};
use diesel::associations::HasTable;
use dotenvy::dotenv;
use uuid::Uuid;
use crate::{Login, NewUserRequest};
use crate::models::Users;
use crate::schema::users::dsl::users;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn verify_login(conn: &mut PgConnection, login: Login) -> bool {

    return true
}

pub async fn verify_token(token: String) -> bool {
    return true
}


pub fn insert_new_user(conn: &mut PgConnection, new_user: NewUserRequest) {
    let uid = format!("{}", Uuid::new_v4());
    let new_user = Users {
        id: uid,
        email: new_user.email,
        firstname: new_user.firstname,
        lastname: new_user.lastname,
        birthdate: new_user.birthdate,
        username: new_user.username,
    };
    let _ = diesel::insert_into(users::table())
        .values(&new_user);
}

