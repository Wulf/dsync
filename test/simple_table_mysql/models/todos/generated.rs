/* @generated and managed by dsync */

#[allow(unused)]
use crate::diesel::*;
use crate::schema::*;

/// Struct representing a row in table `todos`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Queryable, diesel::Selectable, diesel::QueryableByName)]
#[diesel(table_name=todos, primary_key(id))]
pub struct Todos {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `unsigned`
    pub unsigned: u32,
    /// Field representing column `unsigned_nullable`
    pub unsigned_nullable: Option<u32>,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
    /// Field representing column `type`
    pub type_: String,
    /// Field representing column `smallint`
    pub smallint: i16,
    /// Field representing column `bigint`
    pub bigint: i64,
    /// Field representing column `created_at`
    pub created_at: chrono::NaiveDateTime,
    /// Field representing column `updated_at`
    pub updated_at: chrono::NaiveDateTime,
}

/// Create Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::Insertable)]
#[diesel(table_name=todos)]
pub struct CreateTodos {
    /// Field representing column `unsigned`
    pub unsigned: u32,
    /// Field representing column `unsigned_nullable`
    pub unsigned_nullable: Option<u32>,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
    /// Field representing column `type`
    pub type_: String,
    /// Field representing column `smallint`
    pub smallint: i16,
    /// Field representing column `bigint`
    pub bigint: i64,
}

/// Update Struct for a row in table `todos` for [`Todos`]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, diesel::AsChangeset, PartialEq, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodos {
    /// Field representing column `unsigned`
    pub unsigned: Option<u32>,
    /// Field representing column `unsigned_nullable`
    pub unsigned_nullable: Option<Option<u32>>,
    /// Field representing column `text`
    pub text: Option<String>,
    /// Field representing column `completed`
    pub completed: Option<bool>,
    /// Field representing column `type`
    pub type_: Option<String>,
    /// Field representing column `smallint`
    pub smallint: Option<i16>,
    /// Field representing column `bigint`
    pub bigint: Option<i64>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::NaiveDateTime>,
    /// Field representing column `updated_at`
    pub updated_at: Option<chrono::NaiveDateTime>,
}
