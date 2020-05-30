CREATE TABLE Users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    pwd_hash CHAR(128) NOT NULL,
    pwd_salt CHAR(128) NOT NULL
);
