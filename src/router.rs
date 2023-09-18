use axum::response::Html;

struct WebSite {
    theme: String,
}

pub async fn hello_world() -> Html<&'static str> {
    let website = WebSite {theme: "default".to_string()};
    let theme = match website.theme {
        _ => include_str!("../pages/index.html"),
    };
    Html(theme)
}