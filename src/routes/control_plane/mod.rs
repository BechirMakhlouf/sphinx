mod users;

pub fn get_router() -> axum::Router {
    axum::Router::new().nest("/users", users::get_router())
}
