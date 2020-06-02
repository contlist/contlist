table! {
    contacts (id) {
        id -> Int8,
        username -> Varchar,
        contact_name -> Varchar,
        phone -> Varchar,
    }
}

table! {
    users (username) {
        username -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(contacts -> users (username));

allow_tables_to_appear_in_same_query!(
    contacts,
    users,
);
