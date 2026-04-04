use crate::app::{AppState, Command};
use axum::{
    Router,
    extract::{State, WebSocketUpgrade, ws::{Message, WebSocket}},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use tower_http::cors::CorsLayer;

const WEBUI_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/webui.html"));

/// Shared state for the web server.
pub struct WebState {
    pub app: Arc<RwLock<AppState>>,
    pub cmd_tx: Sender<Command>,
}

/// Build and return the Axum router.
pub fn create_router(web_state: Arc<WebState>) -> Router {
    Router::new()
        .route("/", get(serve_webui))
        .route("/api/state", get(get_state))
        .route("/api/command", post(post_command))
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(web_state)
}

async fn serve_webui() -> Html<&'static str> {
    Html(WEBUI_HTML)
}

async fn get_state(State(ws): State<Arc<WebState>>) -> Response {
    let app = ws.app.read().unwrap();
    let snapshot = app.snapshot();
    axum::Json(snapshot).into_response()
}

async fn post_command(
    State(ws): State<Arc<WebState>>,
    axum::Json(cmd): axum::Json<Command>,
) -> Response {
    match ws.cmd_tx.send(cmd) {
        Ok(()) => axum::Json(serde_json::json!({ "ok": true })).into_response(),
        Err(_) => axum::Json(serde_json::json!({ "ok": false, "error": "channel closed" }))
            .into_response(),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<WebState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: Arc<WebState>) {
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let json = {
            let app = state.app.read().unwrap();
            serde_json::to_string(&app.snapshot()).unwrap()
        };
        if socket.send(Message::Text(json.into())).await.is_err() {
            break;
        }
    }
}

/// Start the web server (blocking, runs forever).
pub fn run_server(web_state: Arc<WebState>, port: u16) {
    let router = create_router(web_state);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let addr = format!("0.0.0.0:{port}");
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        eprintln!("🌐 Web UI: http://localhost:{port}");
        axum::serve(listener, router).await.unwrap();
    });
}
