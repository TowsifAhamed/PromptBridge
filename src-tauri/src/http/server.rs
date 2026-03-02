use crate::models::{PromptRequest, PromptResponse};
use crate::state::SharedState;
use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn spawn_http_server(
    state: SharedState,
    bind: String,
    token: String,
    cors_allowlist: Vec<String>,
) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        rt.block_on(async move {
            let addr: SocketAddr = bind.parse().expect("invalid bind address");
            assert!(
                addr.ip().is_loopback(),
                "http server must bind to loopback only"
            );

            let cors = make_cors_layer(cors_allowlist);
            let app = Router::new()
                .route("/run", post(run_handler).options(options_handler))
                .layer(cors)
                .with_state(HttpState {
                    shared: state,
                    token,
                });

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

async fn options_handler() -> impl IntoResponse {
    StatusCode::NO_CONTENT
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

fn make_cors_layer(allowlist: Vec<String>) -> CorsLayer {
    let mut local_allowlist = allowlist;
    local_allowlist.push("http://localhost".into());
    local_allowlist.push("http://127.0.0.1".into());

    CorsLayer::new()
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_origin(AllowOrigin::predicate(move |origin: &HeaderValue, _req| {
            origin
                .to_str()
                .ok()
                .map(|o| {
                    o.starts_with("http://localhost")
                        || o.starts_with("http://127.0.0.1")
                        || local_allowlist.iter().any(|allowed| allowed == o)
                })
                .unwrap_or(false)
        }))
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
        h.insert(
            "authorization",
            "Bearer abc".parse().expect("valid auth header"),
        );
        assert!(is_authorized(&h, "abc"));
        assert!(!is_authorized(&h, "def"));
    }
}
