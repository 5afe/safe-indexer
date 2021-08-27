CREATE TABLE log_entry (
    safe_address CHAR(42),
    transaction_hash CHAR(66),
    block_number CHAR(8),
    PRIMARY KEY(transaction_hash, block_number)
)
