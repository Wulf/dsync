/* @generated and managed by dsync */

use crate::diesel::*;
use crate::data::schema::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};


type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

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

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, AsChangeset, Default)]
#[diesel(table_name=todos)]
pub struct UpdateTodo {
    pub unsigned: Option<u32>,
    pub text: Option<String>,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}


#[derive(Debug, Serialize)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl Todo {

    pub fn create(db: &mut ConnectionType, item: &CreateTodo) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        insert_into(todos).values(item).get_result::<Self>(db)
    }

    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::data::schema::todos::dsl::*;

        todos.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::data::schema::todos::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = todos.count().get_result(db)?;
        let items = todos.limit(page_size).offset(page * page_size).load::<Self>(db)?;

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
        use crate::data::schema::todos::dsl::*;

        diesel::update(todos.filter(id.eq(param_id))).set(item).get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::data::schema::todos::dsl::*;

        diesel::delete(todos.filter(id.eq(param_id))).execute(db)
    }

}