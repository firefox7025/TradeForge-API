use crate::models::Users;
use crate::schema::users::dsl::users;
use crate::schema::users::{email, username};
use crate::{Login, NewUserRequest};
use bcrypt::verify;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods};
use diesel::{Connection, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use std::env;
use std::time::SystemTime;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn verify_login(conn: &mut PgConnection, login: Login) -> Result<Users, String> {
    info!("{}", format!("Login attempt for {}", login.username_or_email));
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
            match verify(login.password.clone(), &user.password).unwrap() {
                true => {
                    Ok(user)
                },
                false => {
                    Err("Invalid password".to_string())
                }
            }
        }
        None => Err("Invalid username or email".to_string())    ,
    }
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
        .execute(conn);
    if resp != Ok(1) {
        warn!("Error saving new user, username or email already taken");
        return Err("Error saving new user, username or email already taken.".parse().unwrap())
    }
    Ok(new_user)
}

pub fn create_jwt(user_id: Users, secret: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 3600;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    encode(&header, &claims, &encoding_key).unwrap()
}

pub fn decode_jwt(token: &str, secret: &str) -> Option<TokenData<Claims>> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();

    decode::<Claims>(token, &decoding_key, &validation).ok()
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Users,
    exp: usize,
}
