use super::Response;
use axum::extract::{FromRequest, RequestParts};
use axum::{async_trait, Json};
use hyper::http::header;
use hyper::StatusCode;

pub struct AuthTokenExtractor(pub String);

#[async_trait]
impl<B> FromRequest<B> for AuthTokenExtractor
where
    B: Send,
{
    type Rejection = (StatusCode, Json<Response>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let token = req
            .headers()
            .and_then(|headers| headers.get(header::AUTHORIZATION))
            .and_then(|header| header.to_str().ok());

        if let Some(token) = token {
            Ok(Self(token.to_owned()))
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                Json(Response::error("Missing Authorization header")),
            ))
        }
    }
}
