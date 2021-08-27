CREATE TABLE log_entry (
    safe_address VARCHAR(42),
    transaction_hash VARCHAR(66),
    block_number VARCHAR(8),
    PRIMARY KEY(transaction_hash, block_number)
)
