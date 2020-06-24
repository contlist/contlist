CREATE TABLE Users (
    username VARCHAR(32) PRIMARY KEY,
    password_hash VARCHAR(128) NOT NULL,
    password_salt VARCHAR(128) NOT NULL,
    create_timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    change_timestamp TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION update_change_timestamp_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.change_timestamp = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_change_timestamp BEFORE UPDATE
    ON Users FOR EACH ROW EXECUTE PROCEDURE 
    update_change_timestamp_column();

