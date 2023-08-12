use axum::{
    response::Html,
    extract::{
        State,
        Path
    },
    http::StatusCode,
};
use sqlx::PgPool;

struct WebSite {
    theme: String,
}

#[derive(Debug, sqlx::FromRow)]
struct Todo {
  id: i32,
  note: String
}

pub async fn hello_world() -> Html<&'static str> {
    let website = WebSite {theme: "default".to_string()};
    let theme = match website.theme {
        _ => include_str!("../site_pages/default/index.html"),
    };
    Html(theme)
}

pub async fn get_database(State(pool): State<PgPool>, Path(id): Path<i32>) -> String {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, note FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(internal_error);

    format!("{:?}", todos)
}

pub async fn set_database(State(pool): State<PgPool>) -> String {
    let todos = sqlx::query_as::<_, Todo>("INSERT INTO todos(note) VALUES ($1) RETURNING id, note")
        .bind("Yes")
        .fetch_one(&pool)
        .await
        .map_err(internal_error);
    
    format!("{:?}", todos)
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}