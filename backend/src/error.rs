use std::convert::Infallible;
use warp::{hyper::StatusCode, reject::Reject, Rejection, Reply};

use crate::payloads::InternalError;

#[derive(Debug)]
pub struct AnyhowError {
    inner: anyhow::Error,
}

impl Reject for AnyhowError {}

impl AnyhowError {
    pub fn into_reject(e: anyhow::Error) -> Rejection {
        warp::reject::custom(Self { inner: e })
    }

    pub fn inner(&self) -> &anyhow::Error {
        &self.inner
    }
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let reason: String;
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;

    if let Some(e) = err.find::<AnyhowError>() {
        reason = e.inner().to_string();
    } else if err.is_not_found() {
        reason = "NOT_FOUND".to_string();
        code = StatusCode::NOT_FOUND;
    } else {
        reason = "NO REASON".to_string();
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&InternalError {
            reason,
            code: code.as_u16(),
        }),
        code,
    ))
}
