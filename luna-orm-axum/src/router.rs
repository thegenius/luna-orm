use crate::handler;
use crate::handler::PostHandler;
use axum::routing::post;
use axum::Router;
use luna_orm::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

/*
pub fn get_post_router<D, T>(db: &DB<D>, path: impl AsRef<str>) -> Router<D>
where
    T: DeserializeOwned + Serialize + Entity + Send + Clone + 'static,
    D: Database + Clone + Send + Sync + 'static,
{
    let post_handler = PostHandler::<D, T>::new(db);
    //Router::new()
    //    .route(path.as_ref(), post(post_handler))
    //    .with_state(db.clone());
    Router::new().route(path.as_ref(), post(post_handler))
}
*/
