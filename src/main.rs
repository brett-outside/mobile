use actix_web::{App, HttpServer, middleware, web};
use std::sync::Arc;

mod models;
mod api;

use models::MasterServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the server with thread-safe wrapping
    let server = Arc::new(MasterServer::new());
    
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            // Enable TLS middleware
            .wrap(middleware::Logger::default())
            // Add shared state
            .app_data(web::Data::new(server.clone()))
            // Configure routes
            .configure(api::configure_app)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}