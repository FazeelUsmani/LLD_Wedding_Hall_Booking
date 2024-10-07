use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use uuid::Uuid;
use crate::models::venue::{Venue, CreateVenueRequest};

lazy_static! {
    static ref VENUES: Mutex<HashMap<Uuid, Venue>> = Mutex::new(HashMap::new());
}

pub fn get_venue_by_id(id: Uuid) -> Option<Venue> {
    VENUES.lock().unwrap().get(&id).cloned()
}

pub fn create_venue(venue: &CreateVenueRequest) -> Venue {
    let id = Uuid::new_v4();
    let new_venue = Venue {
        id,
        name: venue.name.clone(),
        capacity: venue.capacity,
        hourly_rate: venue.hourly_rate,
        description: venue.description.clone(),
    };
    VENUES.lock().unwrap().insert(id, new_venue.clone());
    new_venue
}