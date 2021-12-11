use std::cell::Cell;

use http::status::StatusCode;
use http::{HeaderValue, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BadRequestError(pub anyhow::Error);

impl warp::reject::Reject for BadRequestError {}

#[allow(unused)]
pub async fn parse_body<T>(body: serde_json::Value) -> Result<T, warp::Rejection>
where
    T: for<'de> Deserialize<'de> + Send,
{
    serde_json::from_value::<T>(body).map_err(|e| {
        log::error!("error: {}", e);
        warp::reject::custom(BadRequestError(anyhow::Error::new(e)))
    })
}

pub fn not_found() -> http::Response<hyper::Body> {
    let body = r#"{"description":"Not found"}"#;
    let mut response = Response::new(body.into());
    *response.status_mut() = StatusCode::NOT_FOUND;
    response.headers_mut().insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    response
}

pub fn bad_request<T>(err: T) -> http::Response<hyper::Body>
where
    T: AsRef<str>,
{
    let body = format!(
        "{{\"description\":\"Bad request\",\"error\":\"{}\"}}",
        err.as_ref()
    );

    let mut response = Response::new(body.into());
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response.headers_mut().insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    response
}

pub fn reply_sequence<T, V>(seq: T) -> impl warp::Reply
where
    T: IntoIterator<Item = V>,
    V: Serialize,
{
    struct Sequence<T>(Cell<Option<T>>);

    impl<T, V> Serialize for Sequence<T>
    where
        T: IntoIterator<Item = V>,
        V: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.collect_seq(self.0.take().unwrap())
        }
    }

    warp::reply::json(&Sequence(Cell::new(Some(seq))))
}
