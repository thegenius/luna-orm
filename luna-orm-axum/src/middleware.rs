use std::future::Future;

use axum::body::Body;
use axum::extract::Request;
use axum::extract::State;
use axum::http::header;
use axum::middleware::from_fn_with_state;
use axum::middleware::Next;
use axum::response::Response;
use axum::Json;
use axum::RequestExt;
use axum::RequestPartsExt;

use axum::body;
use axum::middleware::FromFnLayer;
use luna_orm::prelude::*;
use luna_orm::LunaOrmResult;
use serde_json::Value;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub struct OrmLayer<D>
where
    D: Database + Clone,
{
    db: DB<D>,
}

impl<S, D> Layer<S> for OrmLayer<D>
where
    D: Database + Clone,
{
    type Service = OrmService<S, D>;

    fn layer(&self, inner: S) -> Self::Service {
        OrmService {
            inner,
            db: self.db.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrmService<S, D>
where
    D: Database,
{
    inner: S,
    db: DB<D>,
}

struct OrmRequest {
    url: String,
    command: String,
    payload: Value,
}

// async fn extract_orm_request(req: Request) -> LunaOrmResult<OrmRequest> {
//     // get url path
//     let uri = req.uri();
//     let path = uri.path();

//     // get method
//     let method = req.method();

//     // get sub method in header
//     let command = req.headers().get("x-method");

//     // get content type
//     let content_type: String = req
//         .headers()
//         .get(header::CONTENT_TYPE)
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string();

//     // get json payload
//     if content_type.starts_with("application/json") {}
//     let body = req.into_body();
//     let bytes = body::to_bytes(body, 1024 * 1024).await.unwrap();
//     let payload: Value = serde_json::from_slice(&bytes).unwrap();
//     if !payload.is_object() {}

//     return OrmRequest {};
// }

// impl<S, D> Service<Request> for OrmService<S, D>
// where
//     S: Service<Request>,
//     D: Database + Clone,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }

//     fn call(&mut self, req: Request) -> Self::Future {
//         let (mut parts, body) = req.into_parts();
//         let data = body.collect().await.unwrap().to_bytes();
//         let payload: PostRequest<T> = serde_json::from_slice(&data).unwrap();
//         self.inner.call(req)
//     }
// }
