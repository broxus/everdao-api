pub mod controllers;
mod docs;
pub mod requests;
pub mod responses;
mod utils;

use std::net::SocketAddr;
use std::sync::Arc;

use warp::Filter;

use self::controllers::*;
use crate::services::Services;
use crate::sqlx_client::SqlxClient;

pub use self::utils::*;

pub async fn http_service(
    server_http_address: SocketAddr,
    services: Arc<Services>,
    sqlx_client: SqlxClient,
) {
    let ctx = Context {
        services,
        sqlx_client,
    };

    let api = filters::server(ctx);
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS", "PUT"]);
    let log = warp::log("warp");
    let routes = api.with(log).with(cors);
    warp::serve(routes).run(server_http_address).await;
}

mod filters {
    use warp::filters::BoxedFilter;
    use warp::Filter;

    use super::controllers::{self, Context};
    use crate::api::docs;

    pub fn server(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::any().and(api_v1(ctx).or(healthcheck())).boxed()
    }

    fn healthcheck() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("healthcheck")
            .and(warp::get())
            .and_then(get_healthcheck)
            .boxed()
    }

    async fn get_healthcheck() -> Result<impl warp::Reply, warp::Rejection> {
        Ok(warp::reply::json(&()))
    }

    fn api_v1(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("v1")
            .and(
                swagger()
                    .or(post_proposals_search(ctx.clone()))
                    .or(post_votes_search(ctx.clone()))
                    .or(post_voters_search(ctx)),
            )
            .boxed()
    }

    fn swagger() -> BoxedFilter<(impl warp::Reply,)> {
        let docs = docs::swagger();
        warp::path!("swagger.yaml")
            .and(warp::get())
            .map(move || docs.clone())
            .boxed()
    }

    fn post_proposals_search(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("proposals" / "search")
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::proposals::post_proposals_search)
            .boxed()
    }

    fn post_votes_search(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("votes" / "search")
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::votes::post_votes_search)
            .boxed()
    }

    fn post_voters_search(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path!("voters" / ..)
            .and(warp::path::param::<String>())
            .and(warp::path("search"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::voters::post_voters_search)
            .boxed()
    }

    fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: for<'de> serde::Deserialize<'de> + Send,
    {
        warp::body::json()
    }

    fn with_ctx(
        ctx: Context,
    ) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || ctx.clone())
    }
}
