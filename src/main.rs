use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod db;
mod error;
mod handlers;
mod models;
mod repositories;
mod services;

use api::ApiDoc;
use config::Config;
use db::{create_pool, run_migrations};
use handlers::{chat, projects, runner, tasks};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dev_ai_team=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();
    tracing::info!("Starting server with config: {:?}", config);

    // Create database pool
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("Connected to database");

    // Run migrations
    run_migrations(&pool).await?;

    // Build router
    let app = Router::new()
        // Projects
        .route("/api/projects", get(projects::list_projects))
        .route("/api/projects", post(projects::create_project))
        .route("/api/projects/{id}", get(projects::get_project))
        .route("/api/projects/{id}", patch(projects::update_project))
        .route("/api/projects/{id}", delete(projects::delete_project))
        // Tasks
        .route("/api/projects/{project_id}/tasks", get(tasks::list_tasks))
        .route("/api/projects/{project_id}/tasks", post(tasks::create_task))
        .route("/api/tasks/{id}", get(tasks::get_task))
        .route("/api/tasks/{id}", patch(tasks::update_task))
        .route("/api/tasks/{id}", delete(tasks::delete_task))
        .route("/api/tasks/{id}/assign-agent", post(tasks::assign_agent))
        .route("/api/tasks/{id}/runs", get(tasks::get_task_runs))
        .route("/api/tasks/{task_id}/chat", get(chat::get_task_chat))
        // Runner
        .route("/api/runner/poll", get(runner::poll_task))
        .route("/api/runner/heartbeat", post(runner::heartbeat))
        .route("/api/runner/result", post(runner::submit_result))
        .route("/api/runner/log", post(runner::stream_log))
        // Chat
        .route("/api/projects/{project_id}/chat", get(chat::get_project_chat))
        .route("/api/projects/{project_id}/chat", post(chat::send_chat_message))
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Middleware
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Start server
    let addr: SocketAddr = config.server_addr().parse()?;
    tracing::info!("Server listening on {}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
