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
    use std::pin::Pin;

    use futures::Future;
    use warp::filters::BoxedFilter;
    use warp::Filter;

    use super::controllers::{self, Context};
    use crate::api::docs;

    pub fn server(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::any().and(api_v1(ctx).or(healthcheck())).boxed()
    }

    pub fn healthcheck() -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("healthcheck")
            .and(warp::get())
            .and_then(get_healthcheck)
            .boxed()
    }

    pub fn get_healthcheck(
    ) -> Pin<Box<dyn Future<Output = Result<impl warp::Reply, warp::Rejection>> + Send + 'static>>
    {
        Box::pin(async move { Ok(warp::reply::json(&())) })
    }

    pub fn api_v1(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("v1")
            .and(swagger().or(proposals(ctx.clone())).or(voters(ctx)))
            .boxed()
    }

    pub fn swagger() -> BoxedFilter<(impl warp::Reply,)> {
        let docs = docs::swagger();
        warp::path!("swagger.yaml")
            .and(warp::get())
            .map(move || docs.clone())
            .boxed()
    }

    pub fn proposals(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("proposals")
            .and(post_search_proposals(ctx.clone()).or(post_proposal_votes(ctx.clone())))
            .boxed()
    }

    pub fn voters(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("voters")
            .and(
                post_voters_votes(ctx.clone())
                    .or(post_voter_votes(ctx.clone()))
                    .or(post_voters_proposals(ctx.clone())),
            )
            .boxed()
    }

    pub fn post_search_proposals(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("search")
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::proposals::post_search_proposals)
            .boxed()
    }

    pub fn post_proposal_votes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path::param()
            .and(warp::path("votes"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::proposals::post_proposal_votes)
            .boxed()
    }

    pub fn post_voters_votes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("votes")
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::proposals::post_voters_votes)
            .boxed()
    }

    pub fn post_voter_votes(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path::param()
            .and(warp::path("votes"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::proposals::post_voter_votes)
            .boxed()
    }

    pub fn post_voters_proposals(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path::param()
            .and(warp::path("proposals"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::proposals::post_voters_proposals)
            .boxed()
    }

    fn json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: for<'de> serde::Deserialize<'de> + Send,
    {
        warp::body::json()
    }

    #[allow(unused)]
    fn query<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        warp::query()
    }

    #[allow(unused)]
    fn optional_query<T>() -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Clone
    where
        T: for<'de> serde::Deserialize<'de> + Default + Send + 'static,
    {
        warp::any()
            .and(warp::query().or(warp::any().map(T::default)))
            .unify()
    }

    #[allow(unused)]
    fn optional_param<T>(
    ) -> impl Filter<Extract = (Option<T>,), Error = std::convert::Infallible> + Clone
    where
        T: for<'de> serde::Deserialize<'de> + std::str::FromStr + Send + 'static,
    {
        warp::any()
            .and(
                warp::path::param::<T>()
                    .map(Some)
                    .or(warp::any().map(|| None)),
            )
            .unify()
    }

    #[allow(unused)]
    pub fn default_value<T: Default + Send + 'static>(
    ) -> impl Filter<Extract = (T,), Error = std::convert::Infallible> + Copy {
        warp::any().map(Default::default)
    }

    fn with_ctx(
        ctx: Context,
    ) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || ctx.clone())
    }
}
