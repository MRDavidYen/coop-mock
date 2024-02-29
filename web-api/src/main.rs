use std::sync::Arc;

use api::mocks::handle_mock_request;
use coop_service::{container::AppContainer, errors::CustomError};
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};

pub mod api;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let app = Route::new()
        .nest("/settings", api::endpoints::settings::settings_routes())
        .at("/*", handle_mock_request);

    let container = Arc::new(AppContainer::new().await);

    let mock_handler =
        Arc::new(utils::mock_handler::MockEndpointsHandler::new(container.clone()).await?);

    let app = app
        .with(AddData::new(container))
        .with(AddData::new(mock_handler));

    Ok(Server::new(TcpListener::bind("0.0.0.0:3033"))
        .run(app)
        .await?)
}
