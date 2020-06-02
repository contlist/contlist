CREATE TABLE Users (
    username VARCHAR(32) PRIMARY KEY,
    password_hash VARCHAR(128) NOT NULL
);
