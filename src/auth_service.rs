use crate::models::Users;
use crate::schema::users::dsl::users;
use crate::schema::users::{email, username};
use crate::{Login, NewUserRequest};
use bcrypt::verify;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, SelectableHelper};
use diesel::{Connection, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use std::env;
use log::{error, info, trace, warn};
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn verify_login(conn: &mut PgConnection, login: Login) -> bool {
    let username_or_email = login.username_or_email;

    let query = match username_or_email.contains('@') {
        true => users
            .filter(email.eq(username_or_email))
            .first::<Users>(conn),
        false => users
            .filter(username.eq(username_or_email))
            .first::<Users>(conn),
    };
    match query.ok() {
        Some(user) => {
            verify(login.password, &user.password).unwrap()
        }
        None => false,
    }
}

pub async fn verify_token(token: String) -> bool {
    true
}

pub fn insert_new_user(conn: &mut PgConnection, new_user: NewUserRequest) -> Result<Users, String> {
    let uid = format!("{}", Uuid::new_v4());
    let new_user = Users {
        id: uid,
        email: new_user.email,
        firstname: new_user.firstname,
        lastname: new_user.lastname,
        birthdate: new_user.birthdate,
        username: new_user.username,
        password: new_user.password,
    };

    let resp = diesel::insert_into(users::table())
        .values(&new_user)
        .on_conflict(email)
        .do_nothing()
        .execute(conn)
        .expect("Error inserting user");
    if resp != 1 {
        warn!("Error saving new user, username or email already taken");
        return Err("Error saving new user, username or email already taken.".parse().unwrap())
    }
    assert_eq!(resp, 1);
    Ok(new_user)
}
