use crate::models::{PromptRequest, PromptResponse};
use crate::state::SharedState;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;

pub fn spawn_http_server(state: SharedState, bind: String, token: String) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        rt.block_on(async move {
            let app = Router::new()
                .route("/run", post(run_handler))
                .with_state(HttpState {
                    shared: state,
                    token,
                });

            let addr: SocketAddr = bind.parse().expect("invalid bind address");
            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .expect("bind http server");
            axum::serve(listener, app).await.expect("serve");
        });
    });
}

#[derive(Clone)]
struct HttpState {
    shared: SharedState,
    token: String,
}

async fn run_handler(
    State(state): State<HttpState>,
    headers: HeaderMap,
    Json(payload): Json<PromptRequest>,
) -> impl IntoResponse {
    if !is_authorized(&headers, &state.token) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(PromptResponse {
                ok: false,
                status: "error".into(),
                message: "Unauthorized".into(),
                logs: vec![],
                next_actions: vec!["Set Authorization: Bearer <token>".into()],
            }),
        );
    }

    match state.shared.process_request(payload) {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(PromptResponse {
                ok: false,
                status: "error".into(),
                message: err,
                logs: vec![],
                next_actions: vec![],
            }),
        ),
    }
}

fn is_authorized(headers: &HeaderMap, token: &str) -> bool {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.trim() == format!("Bearer {token}"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::is_authorized;
    use axum::http::HeaderMap;

    #[test]
    fn checks_auth() {
        let mut h = HeaderMap::new();
        h.insert("authorization", "Bearer abc".parse().unwrap());
        assert!(is_authorized(&h, "abc"));
        assert!(!is_authorized(&h, "def"));
    }
}
