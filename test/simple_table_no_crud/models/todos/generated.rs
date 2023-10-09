/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    pub id: i32,
    pub unsigned: u32,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    pub unsigned: u32,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    pub unsigned: Option<u32>,
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

