CREATE TABLE Users (
    username VARCHAR(32) PRIMARY KEY,
    pwd_hash CHAR(128) NOT NULL,
    pwd_salt CHAR(128) NOT NULL
);
