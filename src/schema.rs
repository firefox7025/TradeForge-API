// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        #[max_length = 100]
        email -> Varchar,
        birthdate -> Text,
        #[max_length = 100]
        firstname -> Varchar,
        #[max_length = 100]
        lastname -> Varchar,
        #[max_length = 100]
        username -> Varchar,
        password -> Text,
    }
}
