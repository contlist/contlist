table! {
    contacts (id) {
        id -> Int8,
        user_id -> Int8,
        contact_name -> Varchar,
        phone -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Varchar,
        pwd_hash -> Bpchar,
        pwd_salt -> Bpchar,
    }
}

joinable!(contacts -> users (user_id));

allow_tables_to_appear_in_same_query!(contacts, users,);
