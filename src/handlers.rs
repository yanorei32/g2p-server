use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

/// Root handler - returns index.html
pub async fn index() -> Html<&'static str> {
    Html(include_str!("../assets/index.html"))
}

#[derive(Deserialize)]
pub struct G2pRequest {
    pub text: String,
    pub style: Option<String>,
}

#[derive(Serialize)]
pub struct G2pResponse {
    pub text: String,
}

/// G2P conversion API handler
pub async fn g2p(
    State(state): State<AppState>,
    Json(payload): Json<G2pRequest>,
) -> impl IntoResponse {
    if state.maximum_length != 0 && state.maximum_length < payload.text.chars().count() {
        return (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "text/plain")],
            format!("Maximum length ({}) exceeded", state.maximum_length).into_bytes(),
        )
            .into_response();
    }

    let mut result = crate::g2p::convert_text(&payload.text, &state.tokenizer, &state.detector);

    if payload.style.is_some_and(|s| s == "ko") {
        result = crate::korean::convert_to_korean(&result);
    }

    Json(G2pResponse { text: result }).into_response()
}
