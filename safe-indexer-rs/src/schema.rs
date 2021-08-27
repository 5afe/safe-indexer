table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    log_entry (transaction_hash, block_number) {
        safe_address -> Nullable<Varchar>,
        transaction_hash -> Varchar,
        block_number -> Varchar,
    }
}

table! {
    users (email) {
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    invitations,
    log_entry,
    users,
);
