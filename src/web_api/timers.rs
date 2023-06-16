use deno_core::error::AnyError;
use deno_core::op;

#[op]
async fn op_set_timeout(delay: u64) -> Result<(), AnyError> {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
    Ok(())
}
