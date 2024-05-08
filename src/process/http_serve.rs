use anyhow::Result;
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
        .route("/*path", get(get_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn get_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Result<Response<String>, (StatusCode, String)> {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        Err((
            StatusCode::NOT_FOUND,
            format!("file {} not found)", p.display()),
        ))
    } else if p.is_dir() {
        match handle_dir(&p).await {
            Ok(content) => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "text/html")
                    .body(content)
                    .unwrap();
                Ok(response)
            }
            Err(e) => {
                warn!("Error reading dir {:?}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading dir".to_string(),
                ))
            }
        }
    } else {
        match handle_file(&p).await {
            Ok(content) => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "text/plain")
                    .body(content)
                    .unwrap();
                Ok(response)
            }
            Err(e) => {
                warn!("Error reading file {:?}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error reading file".to_string(),
                ))
            }
        }
    }
}

async fn handle_file(p: &PathBuf) -> Result<String> {
    info!("reading file {:?}", p);
    let content = tokio::fs::read_to_string(p).await?;
    info!("read {} bytes", content.len());
    Ok(content)
}

async fn handle_dir(p: &PathBuf) -> Result<String> {
    info!("reading dir {:?}", p);
    let mut file_list = String::new();

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
                        // println!("file entry {:}", line);
                        file_list.push_str(&line)
                    }
                    Err(e) => warn!("Error: {}", e),
                }
            }
            let content = format!("<html><body>\n{}</body></html>", file_list);
            Ok(content)
        }
        Err(e) => Err(e.into()),
    }
}
