use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::models::{Event, MasterServer, Node};

/// Request for retrieving events since a timestamp
#[derive(Deserialize)]
struct EventsRequest {
    since: DateTime<Utc>,
}

/// Handler for POST /event endpoint
async fn post_event(
    event: web::Json<Event>,
    server: web::Data<Arc<MasterServer>>,
) -> impl Responder {
    match server.log_event(event.into_inner()) {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

/// Handler for GET /events endpoint
async fn get_events(
    query: web::Query<EventsRequest>,
    server: web::Data<Arc<MasterServer>>,
) -> impl Responder {
    let events = server.get_events_since(query.since);
    HttpResponse::Ok().json(events)
}

/// Handler for POST /node endpoint
async fn register_node(
    node: web::Json<Node>,
    server: web::Data<Arc<MasterServer>>,
) -> impl Responder {
    server.register_node(node.into_inner());
    HttpResponse::Created().finish()
}

/// Handler for GET /nodes endpoint
async fn get_nodes(server: web::Data<Arc<MasterServer>>) -> impl Responder {
    let nodes = server.get_active_nodes();
    HttpResponse::Ok().json(nodes)
}

/// Configure the API routes
pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/event")
            .route(web::post().to(post_event))
    )
    .service(
        web::resource("/events")
            .route(web::get().to(get_events))
    )
    .service(
        web::resource("/node")
            .route(web::post().to(register_node))
    )
    .service(
        web::resource("/nodes")
            .route(web::get().to(get_nodes))
    );
}