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
    // todo: Remove senstive headers
    let method = request.method().clone();
    let uri = request.uri().clone();
    let req_headers = request.headers().clone();

    let response = next.run(request).await;
    
    let status = response.status();
    let res_headers = response.headers();

    info_request(RequestLog {
        method: method.to_string(),
        url: uri.to_string(),
        headers: req_headers.clone(),
    })
    .await;

    info_response(ResponseLog {
        status: status.as_u16(),
        headers: res_headers.clone(),
    })
    .await;

    Ok(response)
}
