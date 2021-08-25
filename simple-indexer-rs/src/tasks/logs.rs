use celery::prelude::*;

#[celery::task]
pub async fn check_incoming_eth(safe_address: String) -> TaskResult<String> {
    log::info!("check_incoming_eth called for safe: {}", safe_address);
    Ok(String::from("There was eth transferred"))
}
