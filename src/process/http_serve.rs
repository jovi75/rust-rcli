use anyhow::{anyhow, Result};
use axum::{
    extract::{Path, State},
    http::{header::CONTENT_TYPE, Response, StatusCode},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug, Clone)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} at {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/", get(root_handler))
        .route("/*path", get(get_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn root_handler(
    State(state): State<Arc<HttpServeState>>,
) -> Result<Response<String>, (StatusCode, String)> {
    let content = read_dir(&state.path).await;
    make_response(content, true)
}

async fn get_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, (StatusCode, String)> {
    let p = std::path::Path::new(&state.path).join(path);

    if !p.exists() {
        warn!("{:?} not found", p);
        return Err((StatusCode::NOT_FOUND, format!("file {:?} not found", p)));
    }

    if p.is_dir() {
        let content = read_dir(&p).await;
        return make_response(content, true);
    }

    info!("reading file {:?}", p);
    let content = tokio::fs::read_to_string(p).await.map_err(|e| anyhow!(e));
    make_response(content, false)
}

async fn read_dir(p: &PathBuf) -> Result<String> {
    info!("reading dir {:?}", p);
    let mut content = String::new();

    match std::fs::read_dir(p) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let line = format!(
                            "<a href={:?}>{:?}</a><br/>\n",
                            entry.path(),
                            entry.file_name()
                        );
                        content.push_str(&line)
                    }
                    Err(e) => warn!("Error: {}", e),
                }
            }
            Ok(format!("<html><body>\n{}</body></html>", content))
        }
        Err(e) => Err(e.into()),
    }
}

fn make_response(
    content: Result<String>,
    is_dir: bool,
) -> Result<Response<String>, (StatusCode, String)> {
    match content {
        Ok(content) => {
            info!("content length {} bytes", content.len());
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(
                    CONTENT_TYPE,
                    if is_dir { "text/html" } else { "text/plain" },
                )
                .body(content)
                .unwrap();
            Ok(response)
        }
        Err(e) => {
            warn!("reading error:{:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error reading".to_string(),
            ))
        }
    }
}
