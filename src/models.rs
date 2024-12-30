use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
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