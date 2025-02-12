use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rating {
    pub time: u64,
    pub elo: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub userid: String,
    pub formats: HashMap<String, Vec<Rating>>,
}