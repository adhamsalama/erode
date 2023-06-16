use core::panic;
use std::collections::HashMap;
use std::str::FromStr;

use deno_core::error::AnyError;
use deno_core::serde::ser::SerializeStruct;
use deno_core::{op, serde};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;

struct Response {
    status: u16,
    body: Vec<u8>,
}
// implement serialization for Response
impl serde::Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        // S: Serializer,
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("Response", 2)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("body", &self.body)?;
        state.end()
    }
}

#[op]
async fn op_fetch(
    url: String,
    method: String,
    headers: HashMap<String, Option<String>>,
    body: Option<String>,
) -> Result<Response, AnyError> {
    let client = Client::new();
    let mut headermap = HeaderMap::new();
    for (key, value) in headers {
        if let Some(value) = value {
            let name = HeaderName::from_str(&key).unwrap();
            let value = HeaderValue::from_str(&value).unwrap();
            headermap.insert(name, value);
        }
    }
    let request = if method.to_lowercase() == "get" {
        client.get(url)
    } else if method.to_lowercase() == "post" {
        client.post(url)
    } else if method.to_lowercase() == "put" {
        client.put(url)
    } else if method.to_lowercase() == "patch" {
        client.patch(url)
    } else if method.to_lowercase() == "delete" {
        client.delete(url)
    } else if method.to_lowercase() == "head" {
        client.head(url)
    } else {
        panic!("Method not supported");
    };
    let response = request
        .headers(headermap)
        .body(body.unwrap_or_default())
        .send()
        .await;
    match response {
        Ok(response) => {
            let status = response.status();
            let body = response.bytes().await.unwrap();
            let response_body: Vec<u8> = body.into_iter().collect();
            Ok(Response {
                status: status.as_u16(),
                body: response_body,
            })
        }
        Err(e) => Err(e.into()),
    }
}
