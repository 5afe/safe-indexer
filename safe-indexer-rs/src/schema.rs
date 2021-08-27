table! {
    log_entry (transaction_hash, block_number) {
        safe_address -> Nullable<Bpchar>,
        transaction_hash -> Bpchar,
        block_number -> Bpchar,
    }
}

allow_tables_to_appear_in_same_query!(
    invitations,
    log_entry,
    users,
);
