use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::venue::CreateVenueRequest;
use crate::db::mock_db;

pub async fn get_venue(id: web::Path<Uuid>) -> impl Responder {
    let venue_id = id.into_inner();
    
    match mock_db::get_venue_by_id(venue_id) {
        Some(venue) => HttpResponse::Ok().json(venue),
        None => HttpResponse::NotFound().json("Venue not found"),
    }
}

pub async fn create_venue(venue: web::Json<CreateVenueRequest>) -> impl Responder {
    let created_venue = mock_db::create_venue(&venue);
    HttpResponse::Created().json(created_venue)
}