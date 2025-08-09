use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;

mod database;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // load environment variables from .env file
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let api_services = OpenApiService::new(handlers::TodoApi, "Todo api_services", "1.0.0")
        .server("http://localhost:3000/api/v1");

    let ui = api_services.swagger_ui();
    let spec = api_services.spec();

    let app = Route::new()
        .nest("/api/v1", api_services)
        .nest("/docs", ui)
        .at(
            "/api/v1/spec",
            poem::endpoint::make_sync(move |_| spec.clone()),
        )
        .data(pool)
        .with(Cors::new().allow_origin("*"));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
