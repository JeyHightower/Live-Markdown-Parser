use serde::{Deserialize, Serialize };
use axum::{
    routing::post,
    Json,
    Router,
};
use tower_http::services::ServeDir; 
use std::net::SocketAddr;


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

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("🚀 Markdown Parser running at http://{}", addr );

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




fn parse_markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();

    for line in markdown.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }
        let mut processed_line = if trimmed.starts_with("# "){
            let content = &trimmed[2..];
            format!("<h1>{}</h1>", content)
        } else if trimmed.starts_with("## "){
            let content = &trimmed[3..];
            format!("<h2>{}</h2>", content)
        } else if trimmed.starts_with("- "){
            let content = &trimmed[2..];
            format!("<li>{}</li>", content)
        } else {
            format!("<p>{}</p>", trimmed)
        };

        while let Some(start_idx) = processed_line.find("**"){
            if let Some(end_idx) = processed_line[start_idx + 2..].find("**"){
                let actual_end_idx = start_idx + 2 + end_idx;

                let mut new_line = String::new();
                new_line.push_str(&processed_line[..start_idx]);
                new_line.push_str("<strong>");
                new_line.push_str(&processed_line[start_idx +2..actual_end_idx]);
                new_line.push_str("</strong>");
                new_line.push_str(&processed_line[actual_end_idx +2..]);
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
    Json(RenderResponse { html: compiled_html})
}
