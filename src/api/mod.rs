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
        .allow_headers(vec!["content-type", "authorization", "api-key"])
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
            .and(swagger().or(staking(ctx.clone())).or(transfers(ctx)))
            .boxed()
    }

    pub fn swagger() -> BoxedFilter<(impl warp::Reply,)> {
        let docs = docs::swagger();
        warp::path!("swagger.yaml")
            .and(warp::get())
            .map(move || docs.clone())
            .boxed()
    }

    // /v1/staking get get_lp_price
    pub fn staking(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("staking")
            .and(
                search_stakeholders(ctx.clone())
                    .or(get_main_page_staking(ctx.clone()))
                    .or(post_user_page_staking(ctx.clone()))
                    .or(search_transactions(ctx.clone()))
                    .or(post_graph_tvl(ctx.clone()))
                    .or(post_graph_apr(ctx)),
            )
            .boxed()
    }

    pub fn transfers(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("transfers").and(search_transfers(ctx)).boxed()
    }

    pub fn get_main_page_staking(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path::end()
            .and(warp::get())
            .and(with_ctx(ctx))
            .and_then(controllers::staking::get_main_staking)
            .boxed()
    }

    pub fn post_user_page_staking(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path::end()
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::staking::post_user_staking)
            .boxed()
    }

    pub fn search_stakeholders(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("search")
            .and(warp::path("stakeholders"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::staking::search_stakeholders)
            .boxed()
    }

    pub fn search_transfers(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("search")
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::staking::search_transfers)
            .boxed()
    }

    pub fn search_transactions(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("search")
            .and(warp::path("transactions"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::staking::search_transactions)
            .boxed()
    }

    pub fn post_graph_tvl(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("search")
            .and(warp::path("graph"))
            .and(warp::path("tvl"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::staking::post_graph_tvl)
            .boxed()
    }

    pub fn post_graph_apr(ctx: Context) -> BoxedFilter<(impl warp::Reply,)> {
        warp::path("search")
            .and(warp::path("graph"))
            .and(warp::path("apr"))
            .and(warp::path::end())
            .and(warp::post())
            .and(with_ctx(ctx))
            .and(json_body())
            .and_then(controllers::staking::post_graph_apr)
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
