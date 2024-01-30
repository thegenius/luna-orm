use std::future::Future;

use crate::request::*;
use crate::response::*;
use axum::extract::FromRequest;
use axum::extract::Request;
use axum::extract::State;
use axum::handler::Handler;
use axum::http::Result;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum::Router;
use axum_macros::debug_handler;
use http_body_util::BodyExt;
use luna_orm::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::pin::Pin;

/*
struct ResponseSuccess<T> {
    success: bool,
    data: T
}
struct ResponseFailure {
    success: bool,
    err_code: i16,
    err_msg: String
}

enum Response<T> {
    Success(ResultSuccess<T>),
    Failure(ResultFailure)
}
type Result<T> = std::result::Result<Response<T>, thiserror::Error>

enum InsertRequest {

}

// POST /users
insert(State(database), Json(entity)) -> Result<()>
// POST /users
create(State(database), Json(entity)) -> Result<entity>
// POST /users
upsert(State(database), Json(entity)) -> Result<()>

// PUT /users
update(State(database), Json(UpdateRequest)) -> Result<()>
// PUT /users
change(State(database), Json(ChangeRequest)) -> Result<usize>

// DELETE /users
delete(State(database), Json(primary)) -> Result<()>
// DELETE /users
remove(State(database), Json(primary)) -> Result<entity>
// DELETE /users
purify(State(database), Json(location)) -> Result<usize>

*/

//#[debug_handler]
pub async fn post<D, T>(
    State(mut db): State<DB<D>>,
    Json(payload): Json<PostRequest<T>>,
) -> Result<Json<PostResponse<T>>>
where
    T: Serialize + Entity + Send + Sync,
    D: Database,
{
    match payload {
        PostRequest::Create { mut entity } => {
            db.create(&mut entity).await.unwrap();
            let response = PostResponse::Create { entity };
            Ok(Json(response))
        }
        PostRequest::Insert { entity } => {
            db.insert(&entity).await.unwrap();
            Ok(Json(PostResponse::Insert))
        }
        PostRequest::Upsert { entity } => {
            db.upsert(&entity).await.unwrap();
            Ok(Json(PostResponse::Insert))
        }
    }
}

#[derive(Clone, Debug)]
pub struct PostHandler<D, T>
where
    D: Database + Clone + Send + 'static,
    T: DeserializeOwned + Entity + Send + Clone + 'static,
{
    db: PhantomData<D>,
    data: PhantomData<T>,
}

impl<D, T> Default for PostHandler<D, T>
where
    D: Database + Clone + Send + 'static,
    T: DeserializeOwned + Entity + Send + Clone + 'static,
{
    fn default() -> Self {
        Self {
            db: PhantomData,
            data: PhantomData,
        }
    }
}

impl<S, T> Handler<((),), S> for PostHandler<S, T>
where
    S: Database + Clone + Send + 'static,
    T: DeserializeOwned + Entity + Send + Clone + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: Request, state: S) -> Self::Future {
        Box::pin(async move {
            let (mut parts, body) = req.into_parts();
            let data = body.collect().await.unwrap().to_bytes();
            let payload: PostRequest<T> = serde_json::from_slice(&data).unwrap();
            post_test(payload).await;

            let response: Response = StatusCode::NOT_FOUND.into_response();
            response
        })
    }
}

pub async fn post_test<T>(payload: PostRequest<T>) -> bool
where
    T: Entity + Send + Sync,
{
    true
    /*
        match payload {
        PostRequest::Create { mut entity } => {
            db.create(&mut entity).await.unwrap();
            let response = PostResponse::Create { entity };
            Ok(Json(response))
        }
        PostRequest::Insert { entity } => {
            db.insert(&entity).await.unwrap();
            Ok(Json(PostResponse::Insert))
        }
        PostRequest::Upsert { entity } => {
            db.upsert(&entity).await.unwrap();
            Ok(Json(PostResponse::Insert))
        }
    }
    */
}
pub async fn put<D, M, P, L>(
    State(mut db): State<DB<D>>,
    Json(payload): Json<PutRequest<M, P, L>>,
) -> Result<Json<PutResponse>>
where
    M: Serialize + Mutation,
    P: Serialize + Primary,
    L: Serialize + Location,
    D: Database,
{
    match payload {
        PutRequest::Update { mutation, primary } => {
            db.update(&mutation, &primary).await.unwrap();
            let response = PutResponse::Update;
            Ok(Json(response))
        }
        PutRequest::Change { mutation, location } => {
            let count = db.change(&mutation, &location).await.unwrap();
            Ok(Json(PutResponse::Change { count }))
        }
    }
}

pub async fn delete<D, P, L, S, SE>(
    State(mut db): State<DB<D>>,
    Json(payload): Json<DeleteRequest<P, L, S>>,
) -> Result<Json<DeleteResponse<SE>>>
where
    P: Serialize + Primary,
    L: Serialize + Location,
    S: Serialize + Selection,
    SE: Serialize + SelectedEntity + Send + Unpin,
    D: Database,
{
    match payload {
        DeleteRequest::Delete { primary } => {
            db.delete(&primary).await.unwrap();
            let response = DeleteResponse::Delete;
            Ok(Json(response))
        }
        DeleteRequest::Remove { primary, selection } => {
            let entity: Option<SE> = db.remove(&primary, &selection).await.unwrap();
            Ok(Json(DeleteResponse::Remove { entity }))
        }
        DeleteRequest::Purify { location } => {
            let count = db.purify(&location).await.unwrap();
            Ok(Json(DeleteResponse::Purify { count }))
        }
    }
}

pub fn generate_router<D, P, E>(path: P)
where
    P: AsRef<str>,
    D: Database,
    E: Entity + Serialize,
{
    /*
    let router = Router::new().route(
        path.as_ref(),
        axum::routing::post(post::<SqliteDatabase, E>),
    );
    */
}
