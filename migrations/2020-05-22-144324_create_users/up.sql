CREATE TABLE Users (
    username VARCHAR(32) PRIMARY KEY,
    password_hash VARCHAR(128) NOT NULL,
    password_salt VARCHAR(128) NOT NULL
);
