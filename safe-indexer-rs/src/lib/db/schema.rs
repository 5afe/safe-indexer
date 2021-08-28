use diesel::table;

table! {
    log_entry (transaction_hash, block_number) {
        safe_address -> Nullable<Bpchar>,
        transaction_hash -> Bpchar,
        block_number -> Bpchar,
    }
}
