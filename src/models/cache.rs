use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::models::session::Session;
use crate::models::user::User;

type UserId = uuid::Uuid;

/// The application cache. Stores data that's local to the application.
/// This is often a good place to prototype data structures before moving
/// to a durable storage system such as Redis or Postgres.
pub struct Cache {
    pub users: Mutex<HashSet<User>>,
    pub sessions: Mutex<HashMap<UserId, Session>>,
}
