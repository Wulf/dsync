/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use crate::models::common::*;

/// Struct representing a row in table `todo`
#[derive(Debug, Clone, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=todo, primary_key(id))]
pub struct Todo {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
    /// Field representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Create Struct for a row in table `todo` for [`Todo`]
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=todo)]
pub struct CreateTodo {
    /// Field representing column `id`
    pub id: i32,
    /// Field representing column `text`
    pub text: String,
    /// Field representing column `completed`
    pub completed: bool,
    /// Field representing column `created_at`
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Field representing column `updated_at`
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Update Struct for a row in table `todo` for [`Todo`]
#[derive(Debug, Clone, AsChangeset, PartialEq, Default)]
#[diesel(table_name=todo)]
pub struct UpdateTodo {
    /// Field representing column `text`
    pub text: Option<String>,
    /// Field representing column `completed`
    pub completed: Option<bool>,
    /// Field representing column `created_at`
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Field representing column `updated_at`
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Todo {
    /// Insert a new row into `todo` with a given [`CreateTodo`]
    pub fn create(db: &mut ConnectionType, item: &CreateTodo) -> QueryResult<Self> {
        use crate::schema::todo::dsl::*;

        insert_into(todo).values(item).get_result::<Self>(db)
    }

    /// Get a row from `todo`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::todo::dsl::*;

        todo.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::todo::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = todo.count().get_result(db)?;
        let items = todo.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Update a row in `todo`, identified by the primary key with [`UpdateTodo`]
    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateTodo) -> QueryResult<Self> {
        use crate::schema::todo::dsl::*;

        diesel::update(todo.filter(id.eq(param_id))).set(item).get_result(db)
    }

    /// Delete a row in `todo`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::todo::dsl::*;

        diesel::delete(todo.filter(id.eq(param_id))).execute(db)
    }
}
