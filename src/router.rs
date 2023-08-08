use axum::response::Html;
use axum::{routing::get, Router};

struct WebSite {
    theme: String,
}

pub fn build_router() -> Router {


    Router::new().route("/", get(hello_world))
}

pub async fn hello_world() -> Html<&'static str> {
    let website = WebSite {theme: "default".to_string()};
    let theme = match website.theme {
        _ => include_str!("../site_pages/default/index.html"),
    };
    Html(theme)
}