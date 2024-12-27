-- Your SQL goes here
CREATE TABLE users (
       id text PRIMARY KEY,
       email varchar(100) NOT NULL,
       birthdate TEXT NOT NULL,
       firstname varchar(100) NOT NULL,
       lastname varchar(100) NOT NULL,
       username varchar(100) NOT NULL,
       password TEXT NOT NULL
)
