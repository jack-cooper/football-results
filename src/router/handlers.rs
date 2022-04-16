use crate::{cache::Cached, error::ReportError, football::FootballResults};
use axum::{extract::Extension, response::IntoResponse, Json};
use color_eyre::Report;
use hyper::{Body, Response};
use reqwest::Client;
use serde::Serialize;

#[tracing::instrument(skip_all)]
pub(crate) async fn get_football_data(
    client: Extension<Client>,
    cached: Extension<Cached<FootballResults>>,
) -> Result<impl IntoResponse, ReportError> {
    #[derive(Serialize)]
    struct Response {
        football_results: String,
    }

    let FootballResults(football_results) = cached
        .get_cached(|| {
            Box::pin(async move {
                let results = crate::football::fetch_results(&client).await?;
                Ok::<_, Report>(FootballResults(results))
            })
        })
        .await?;

    Ok(Json(Response { football_results }))
}

#[tracing::instrument]
pub(crate) async fn health_check() -> Response<Body> {
    Default::default()
}

#[tracing::instrument]
pub(crate) async fn hello_world() -> &'static str {
    "Hello Football API World!"
}
