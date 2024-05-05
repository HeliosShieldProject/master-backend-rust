use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::logger::{
    info_request, info_response,
    types::{RequestLog, ResponseLog},
};

pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let mut req_headers = request.headers().clone();
    req_headers.remove("authorization");

    info_request(RequestLog {
        method: method.to_string(),
        url: uri.to_string(),
        headers: req_headers.clone(),
    })
    .await;

    let response = next.run(request).await;
    let status = response.status();
    let res_headers = response.headers();

    info_response(ResponseLog {
        status: status.as_u16(),
        headers: res_headers.clone(),
    })
    .await;

    Ok(response)
}
