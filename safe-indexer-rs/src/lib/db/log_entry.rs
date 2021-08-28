use super::schema::log_entry;

#[derive(Queryable, Insertable)]
#[table_name = "log_entry"]
pub struct LogEntry<'a> {
    pub safe_address: &'a String,
    transaction_hash: &'a String,
    block_number: &'a String,
}
