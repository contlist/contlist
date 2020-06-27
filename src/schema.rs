table! {
    contacts (id) {
        id -> Int8,
        username -> Varchar,
        contact_name -> Varchar,
        phone_number -> Varchar,
        create_timestamp -> Timestamptz,
        change_timestamp -> Nullable<Timestamptz>,
    }
}

table! {
    users (username) {
        username -> Varchar,
        password_hash -> Varchar,
        password_salt -> Varchar,
        create_timestamp -> Timestamptz,
        change_timestamp -> Nullable<Timestamptz>,
    }
}

joinable!(contacts -> users (username));

allow_tables_to_appear_in_same_query!(contacts, users,);
