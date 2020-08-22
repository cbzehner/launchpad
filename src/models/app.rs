use std::collections::HashSet;
use std::sync::Mutex;

use crate::models::user::User;

pub struct AppState {
    pub users: Mutex<HashSet<User>>,
}
