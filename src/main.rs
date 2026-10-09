use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

//Incoming payload from the browser editor
#[derive(Deserialize)]
pub struct RenderRequest {
    pub markdown: String,
}

//Outgoing payload sent back to the browser preview
#[derive(Serialize)]
pub struct RenderResponse {
    pub html: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/render", post(render_handler))
        .fallback_service(ServeDir::new("static"));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(4000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Markdown Parser running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn parse_markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let escaped = escape_html(trimmed);

        let mut processed_line = if escaped.starts_with("# ") {
            format!("<h1>{}</h1>", &escaped[2..])
        } else if escaped.starts_with("## ") {
            format!("<h2>{}</h2>", &escaped[3..])
        } else if escaped.starts_with("- ") {
            format!("<li>{}</li>", &escaped[2..])
        } else {
            format!("<p>{}</p>", escaped)
        };

        while let Some(start_idx) = processed_line.find("**") {
            if let Some(end_idx) = processed_line[start_idx + 2..].find("**") {
                let actual_end_idx = start_idx + 2 + end_idx;

                let mut new_line = String::new();
                new_line.push_str(&processed_line[..start_idx]);
                new_line.push_str("<strong>");
                new_line.push_str(&processed_line[start_idx + 2..actual_end_idx]);
                new_line.push_str("</strong>");
                new_line.push_str(&processed_line[actual_end_idx + 2..]);
                processed_line = new_line;
            } else {
                break;
            }
        }
        html_output.push_str(&processed_line);
        html_output.push('\n');
    }

    html_output
}

async fn render_handler(Json(payload): Json<RenderRequest>) -> Json<RenderResponse> {
    let compiled_html = parse_markdown_to_html(&payload.markdown);
    Json(RenderResponse {
        html: compiled_html,
    })
}
