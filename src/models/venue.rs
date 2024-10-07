use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub capacity: i32,
    pub hourly_rate: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateVenueRequest {
    pub name: String,
    pub capacity: i32,
    pub hourly_rate: f64,
    pub description: String,
}