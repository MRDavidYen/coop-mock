use endpoint_handler::utils::mock_handler::{DatabaseMockHandlerImpl, MockEndpointHandler};
use poem::{handler, http::StatusCode, web::Data, IntoResponse, Request};
use std::sync::Arc;

#[handler]
pub async fn handle_mock_request(
    req: &Request,
    mocks_handler: Data<&Arc<DatabaseMockHandlerImpl>>,
) -> impl IntoResponse {
    let mocks_handler = mocks_handler.clone();

    let response = mocks_handler.handle_mock_request(req).await;

    if response.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }

    response.unwrap()
}
