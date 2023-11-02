/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use crate::models::common::*;

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name=todo, primary_key(id))]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name=todo)]
pub struct CreateTodo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, AsChangeset, Default)]
#[diesel(table_name=todo)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Todo {
    pub fn create(db: &mut ConnectionType, item: &CreateTodo) -> QueryResult<Self> {
        use crate::schema::todo::dsl::*;

        insert_into(todo).values(item).get_result::<Self>(db)
    }

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

    pub fn update(db: &mut ConnectionType, param_id: i32, item: &UpdateTodo) -> QueryResult<Self> {
        use crate::schema::todo::dsl::*;

        diesel::update(todo.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::todo::dsl::*;

        diesel::delete(todo.filter(id.eq(param_id))).execute(db)
    }
}
