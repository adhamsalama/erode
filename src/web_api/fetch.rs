use deno_core::error::AnyError;
use deno_core::op;

#[op]
async fn op_fetch(url: String) -> Result<String, AnyError> {
    let res = reqwest::get(url)
        .await?
        // .json::<HashMap<String, String>>()
        .text()
        .await?;
    Ok(res)
}
