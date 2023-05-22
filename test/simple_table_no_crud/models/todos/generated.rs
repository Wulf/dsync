/* This file is generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todo {
    pub id: i32,
    pub unsigned: u32,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct CreateTodo {
    pub unsigned: u32,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name=todos)]
pub struct UpdateTodo {
    pub unsigned: Option<u32>,
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

