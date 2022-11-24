use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub phone: String,
    pub created_at: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
pub struct Conversation {
    pub id: String,
    pub user_id: String,
    pub room_id: String,
    pub content: String,
    pub created_at: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub last_message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub phone: String,
}