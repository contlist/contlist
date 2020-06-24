CREATE TABLE Contacts (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(32) NOT NULL REFERENCES Users ON DELETE CASCADE,
    contact_name VARCHAR(32) NOT NULL,
    phone VARCHAR(15) NOT NULL CONSTRAINT phone_chk CHECK (phone SIMILAR TO '\+?[0-9]+'),
    create_timestamp TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    change_timestamp TIMESTAMPTZ
);

CREATE TRIGGER update_contacts_change_timestamp BEFORE UPDATE
    ON Contacts FOR EACH ROW EXECUTE PROCEDURE 
    update_change_timestamp_column();
