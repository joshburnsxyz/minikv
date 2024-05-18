use actix_web::web;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub store: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> web::Data<Self> {
        let state = web::Data::new(AppState {
            store: Mutex::new(HashMap::new()),
        });
        return state;
    }
}
